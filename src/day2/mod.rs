use std::fs::read_to_string;
use crate::day2::RPS::{Paper, Rock, Scissors};

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RPS {
    fn from(s: &str) -> RPS {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!(),
        }
    }
}

impl RPS {
    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn outcome(&self, other: &Self ) -> usize {
        match (self, other) {
            (Rock, Rock)|(Paper,Paper)|(Scissors,Scissors) => 3,
            (Rock, Scissors)|(Scissors, Paper)|(Paper, Rock) => 6,
            _ => 0
        }
    }
}

fn ex1(input: String) -> usize {
    input.lines()
        .map(map_line_to_pair)
        .map(|(other, me)| me.score() + me.outcome(&other))
        .sum()
}

fn map_line_to_pair(line: &str) -> (RPS, RPS) {
    let (other, me) = split(line);
    (RPS::from(other), RPS::from(me))
}

fn ex2(input: String) -> usize {
    input.lines()
        .map(map_to_rps_and_result)
        .map(|(other, result)| {
            let me = find_me(&other, result);
            (other, me)
        })
        .map(|(other, me)| me.score() + me.outcome(&other))
        .sum()
}

fn map_to_rps_and_result(line: &str) -> (RPS, usize) {
    let (other, result) = split(line);
    (RPS::from(other), result_to_outcome_score(result))
}

fn split(line: &str) -> (&str, &str) {
    match line.split(" ").collect::<Vec<&str>>()[..] {
        [other, result] => (other, result),
        _ => panic!(),
    }
}

fn result_to_outcome_score(result: &str) -> usize {
    match result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    }
}

fn find_me(other: &RPS, outcome: usize)->RPS {
    [Rock, Paper, Scissors]
        .into_iter()
        .find(|it| it.outcome(other) == outcome)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        assert_eq!(15, ex1(read_to_string("src/day2/testInput").unwrap()));
    }

    #[test]
    fn answer1() {
        println!("{}", ex1(read_to_string("src/day2/input").unwrap()));
    }

    #[test]
    fn test_ex2() {
        assert_eq!(12, ex2(read_to_string("src/day2/testInput").unwrap()));
    }

    #[test]
    fn answer2() {
        println!("{}", ex2(read_to_string("src/day2/input").unwrap()));
    }
}