use crate::tests;
use regex::Regex;
use std::mem::take;

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn do_monkey_business(&self, items: Vec<usize>, worry_factor: usize) -> Vec<(usize, usize)> {
        items
            .into_iter()
            .map(|item| self.operation.apply(item))
            .map(|item| item / worry_factor)
            .map(|item| (self.test.test(&item), item))
            .collect()
    }
}

#[derive(Debug)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Multiply(rhs) => item * rhs,
            Operation::Add(rhs) => item + rhs,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: usize,
    true_to: usize,
    false_to: usize,
}

impl Test {
    fn test(&self, item: &usize) -> usize {
        match item % self.divisible_by == 0 {
            true => self.true_to,
            false => self.false_to,
        }
    }
}

fn ex1(input: String) -> usize {
    compute_monkey_business(input, 20, 3)
}

fn ex2(input: String) -> usize {
    compute_monkey_business(input, 10_000, 1)
}

fn compute_monkey_business(input: String, iterations: i32, worry_factor: usize) -> usize {
    let mut monkeys = get_monkeys(input);

    let modulos: usize = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    let mut monkey_businesses = vec![0; monkeys.len()];

    let monkey_range = 0..monkeys.len();
    for _ in 0..iterations {
        for i in monkey_range.clone() {
            let items = take(&mut monkeys.get_mut(i).unwrap().items);
            let monkey = &monkeys[i];

            let businessed_items = monkey.do_monkey_business(items, worry_factor);
            let monkey_business = monkey_businesses.get_mut(i).unwrap();
            *monkey_business += businessed_items.len();

            for (to, item) in businessed_items {
                monkeys.get_mut(to).unwrap().items.push(item % modulos);
            }
        }
    }
    monkey_businesses.sort();
    monkey_businesses.pop().unwrap() * monkey_businesses.pop().unwrap()
}

fn get_monkeys(input: String) -> Vec<Monkey> {
    let split = input.split("\r\n\r\n");

    let monkey_regex = Regex::new(r"Monkey (\d+):").unwrap();

    let mut monkeys = vec![];
    for raw_monkey in split {
        let monkey_nr: usize = monkey_regex.captures(raw_monkey).unwrap()[1]
            .parse()
            .unwrap();

        monkeys.insert(
            monkey_nr,
            Monkey {
                items: get_starting_items(raw_monkey),
                operation: get_operation(raw_monkey),
                test: get_test(raw_monkey),
            },
        )
    }
    monkeys
}

fn get_starting_items(raw_monkey: &str) -> Vec<usize> {
    let items_regex = Regex::new(r"Starting items: (\d+(, ?\d+)*)").unwrap();

    let captures = items_regex.captures(raw_monkey).unwrap();

    captures[1]
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn get_operation(raw_monkey: &str) -> Operation {
    let operation_regex = Regex::new(r"Operation: new = old ([+*]) (old|\d+)").unwrap();

    let captures = operation_regex.captures(raw_monkey).unwrap();

    match (&captures[1], &captures[2]) {
        ("+", rhs) => Operation::Add(rhs.parse().unwrap()),
        ("*", "old") => Operation::Square,
        ("*", rhs) => Operation::Multiply(rhs.parse().unwrap()),
        _ => panic!(),
    }
}

fn get_test(raw_monkey: &str) -> Test {
    let test_regex = Regex::new(r"Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();

    let captures = test_regex.captures(raw_monkey).unwrap();

    Test {
        divisible_by: captures[1].parse().unwrap(),
        true_to: captures[2].parse().unwrap(),
        false_to: captures[3].parse().unwrap(),
    }
}

tests!(10605, 2713310158);
