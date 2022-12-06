use std::collections::HashSet;

use crate::tests;

fn ex1(input: String) -> Vec<usize> {
    input.lines().map(find_marker::<4>).collect()
}

fn ex2(input: String) -> Vec<usize> {
    input.lines().map(find_marker::<14>).collect()
}

fn find_marker<const MARKER_LENGTH: usize>(line: &str) -> usize {
    line.as_bytes()
        .windows(MARKER_LENGTH)
        .enumerate()
        .find(|(_, window)| HashSet::<&u8>::from_iter(window.iter()).len() == MARKER_LENGTH)
        .map(|(i, _)| i + MARKER_LENGTH)
        .unwrap()
}

tests!(vec![7, 5, 6, 10, 11], vec![19, 23, 23, 29, 26]);
