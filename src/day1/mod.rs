use crate::tests;

fn ex1(input: String) -> usize {
    input.split("\r\n\r\n").map(sum_part).max().unwrap()
}

fn sum_part(part: &str) -> usize {
    part.lines()
        .map(|number| number.trim().parse::<usize>().unwrap())
        .sum()
}

fn ex2(input: String) -> usize {
    let mut elves = input
        .split("\r\n\r\n")
        .map(sum_part)
        .collect::<Vec<usize>>();

    elves.sort();

    elves.into_iter().rev().take(3).sum()
}

tests!(24000, 45000);
