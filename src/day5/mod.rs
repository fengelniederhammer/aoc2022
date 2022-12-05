use std::collections::HashMap;
use std::fs::read_to_string;
use std::vec::Drain;

use regex::Regex;

use crate::tests;

fn ex1(input: String) -> String {
    let (mut stacks, commands) = extract_stacks_and_commands(input);

    for (n, from, to) in commands {
        let mut moved = pop_n(&mut stacks, n, &from).rev().collect::<Vec<_>>();
        stacks
            .entry(to)
            .and_modify(|stack| stack.append(&mut moved));
    }

    get_tops_items(stacks)
}

fn extract_stacks_and_commands(
    input: String,
) -> (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut split = input.split("\r\n\r\n");

    let stacks = to_stacks(split.next().unwrap());
    let commands = to_commands(split.next().unwrap());

    (stacks, commands)
}

fn to_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let mut lines_chars = input.lines().rev().map(|line| line.chars());
    let stack_numbers = lines_chars
        .next()
        .unwrap()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize));

    let lines = lines_chars.map(Iterator::collect::<Vec<_>>);

    stack_numbers
        .map(|(i, n)| (n, get_items(i, lines.clone())))
        .collect()
}

fn get_items(index: usize, lines: impl Iterator<Item = Vec<char>>) -> Vec<char> {
    lines
        .map(|items| items[index])
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn to_commands(input: &str) -> Vec<(usize, usize, usize)> {
    let regex = Regex::new(r"move (?P<n>\d+) from (?P<from>\d) to (?P<to>\d)").unwrap();
    input
        .lines()
        .map(|line| regex.captures(line).unwrap())
        .map(|captures| {
            (
                to_usize(&captures["n"]),
                to_usize(&captures["from"]),
                to_usize(&captures["to"]),
            )
        })
        .collect()
}

fn to_usize(x: &str) -> usize {
    x.parse::<usize>().unwrap()
}

fn pop_n<'a>(stacks: &'a mut HashMap<usize, Vec<char>>, n: usize, from: &usize) -> Drain<'a, char> {
    let from_stack = stacks.get_mut(from).unwrap();

    from_stack.drain((from_stack.len() - n)..)
}

fn get_tops_items(stacks: HashMap<usize, Vec<char>>) -> String {
    let mut iter = stacks.into_iter().collect::<Vec<_>>();
    iter.sort_by_key(|(n, _)| *n);
    let tops = iter.into_iter().map(|(_, mut stack)| stack.pop().unwrap());
    String::from_iter(tops)
}

fn ex2(input: String) -> String {
    let (mut stacks, commands) = extract_stacks_and_commands(input);

    for (n, from, to) in commands {
        let mut moved = pop_n(&mut stacks, n, &from).collect::<Vec<_>>();
        stacks
            .entry(to)
            .and_modify(|stack| stack.append(&mut moved));
    }

    get_tops_items(stacks)
}

tests!("CMZ", "MCD");
