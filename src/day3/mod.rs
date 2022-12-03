use std::fs::read_to_string;
use std::str::Lines;

fn ex1(input: String) -> usize {
    input.lines()
        .map(split_line)
        .map(find_duplicate)
        .map(score)
        .sum()
}

fn split_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_duplicate((left, right): (&str, &str)) -> char {
    for c in left.chars() {
        if right.contains(c) {
            return c;
        }
    }

    panic!();
}

fn score(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}

fn ex2(input: String) -> usize {
    let groups = GroupIter {
        inner: input.lines()
    };

    groups.map(find_batch)
        .map(score)
        .sum()
}

struct GroupIter<'a>{
    inner: Lines<'a>,
}

impl<'a> Iterator for GroupIter<'a> {
    type Item = (&'a str, &'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.inner.next()?, self.inner.next()?,self.inner.next()?))
    }
}

fn find_batch((left, middle, right): (&str, &str, &str)) -> char {
    for char in left.chars() {
        if middle.contains(char) && right.contains(char) {
            return char;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        assert_eq!(157, ex1(read_to_string("src/day3/testInput").unwrap()));
    }

    #[test]
    fn answer1() {
        println!("{}", ex1(read_to_string("src/day3/input").unwrap()));
    }

    #[test]
    fn test_ex2() {
        assert_eq!(70, ex2(read_to_string("src/day3/testInput").unwrap()));
    }

    #[test]
    fn answer2() {
        println!("{}", ex2(read_to_string("src/day3/input").unwrap()));
    }
}