use std::fs::read_to_string;

fn answer_ex1() -> usize {
    ex1(read_to_string("src/day1/input").unwrap())
}

fn ex1(input: String) -> usize {
    input.split("\r\n\r\n")
        .map(sum_part)
        .max().unwrap()
}

fn sum_part(part: &str) -> usize {
    part.lines()
        .map(|number| number.trim().parse::<usize>().unwrap())
        .sum()
}

fn ex2(input: String) -> usize {
    let mut elves = input.split("\r\n\r\n")
        .map(sum_part)
        .collect::<Vec<usize>>();

    elves.sort();

    elves.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        assert_eq!(24000, ex1(read_to_string("src/day1/testInput").unwrap()));
    }

    #[test]
    fn answer1() {
        println!("{}", answer_ex1());
    }

    #[test]
    fn test_ex2() {
        assert_eq!(45000, ex2(read_to_string("src/day1/testInput").unwrap()));
    }

    #[test]
    fn answer2() {
        println!("{}", ex2(read_to_string("src/day1/input").unwrap()));
    }
}