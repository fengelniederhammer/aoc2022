use std::fs::read_to_string;
use std::ops::RangeInclusive;

use crate::tests;

fn ex1(input: String) -> usize {
    input.lines()
        .map(split_line)
        .filter(|(left, right)| range_contains_range(left, right) || range_contains_range(right, left))
        .count()
}

fn split_line(line: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let mut split = line.split(",");

    (to_range(split.next().unwrap()), to_range(split.next().unwrap()))
}

fn to_range(input: &str) -> RangeInclusive<usize> {
    let mut split = input.split("-");
    split.next().unwrap().parse::<usize>().unwrap()..=split.next().unwrap().parse::<usize>().unwrap()
}

fn range_contains_range(left: &RangeInclusive<usize>, right: &RangeInclusive<usize>) -> bool {
    left.end() >= right.end() && left.start() <= right.start()
}

fn ex2(input: String) -> usize {
    input.lines()
        .map(split_line)
        .filter(ranges_overlap)
        .count()
}

fn ranges_overlap((left, right): &(RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    right.contains(left.end())
        || right.contains(left.start())
        || left.contains(right.start())
        || left.contains(right.end())
}

tests!(2, 4);
