use std::cmp::Ordering;

use json::JsonValue;

use crate::tests;

fn ex1(input: String) -> usize {
    input
        .split("\r\n\r\n")
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .filter(|(_, pair)| is_correctly_ordered_pair(pair))
        .map(|(i, _)| i)
        .sum()
}

fn ex2(mut input: String) -> usize {
    input.push_str("\n[[2]]\n[[6]]");

    let mut lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();

    lines.sort_by(|left, right| compare_arrays(&as_json_array(left), &as_json_array(right)));

    (lines.iter().position(|line| *line == "[[2]]").unwrap() + 1)
        * (lines.iter().position(|line| *line == "[[6]]").unwrap() + 1)
}

fn is_correctly_ordered_pair(pair: &str) -> bool {
    let (left, right) = split(pair);

    let left = as_json_array(left);
    let right = as_json_array(right);

    compare_arrays(&left, &right) == Ordering::Less
}

fn compare(item: (&JsonValue, &JsonValue)) -> Ordering {
    match item {
        (JsonValue::Number(left), JsonValue::Number(right)) => {
            f32::from(*left).total_cmp(&(*right).into())
        }
        (JsonValue::Number(left), JsonValue::Array(right)) => {
            compare_arrays(&vec![JsonValue::Number(*left)], right)
        }
        (JsonValue::Array(left), JsonValue::Number(right)) => {
            compare_arrays(left, &vec![JsonValue::Number(*right)])
        }
        (JsonValue::Array(left), JsonValue::Array(right)) => compare_arrays(left, right),
        _ => panic!(),
    }
}

fn compare_arrays(left: &[JsonValue], right: &[JsonValue]) -> Ordering {
    let length_comparison = left.len().cmp(&right.len());

    for item in left.iter().zip(right) {
        match compare(item) {
            Ordering::Equal => {}
            not_equal => return not_equal,
        }
    }
    length_comparison
}

fn split(pair: &str) -> (&str, &str) {
    let mut lines = pair.lines();
    (lines.next().unwrap(), lines.next().unwrap())
}

fn as_json_array(left: &str) -> Vec<JsonValue> {
    match json::parse(left).unwrap() {
        JsonValue::Array(array) => array,
        _ => panic!(),
    }
}

tests!(13, 140);
