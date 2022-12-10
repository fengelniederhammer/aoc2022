use crate::day10::Command::{AddX, Noop};
use regex::Regex;
use std::str::Lines;

use crate::tests;

#[derive(Debug)]
enum Command {
    Noop,
    AddX(isize),
}

impl Command {
    fn cycles_to_complete(&self) -> usize {
        match self {
            Noop => 1,
            AddX(_) => 2,
        }
    }

    fn apply(&self, x: &mut isize) {
        match self {
            Noop => {}
            AddX(value) => *x += value,
        }
    }
}

fn ex1(input: String) -> isize {
    let mut commands = to_commands(input.lines());

    let mut x = 1;
    let mut signal_strength = 0;
    let mut command = commands.next().unwrap();
    let mut cycles_for_current_command = 0;

    let cycles_to_look_at = vec![20, 60, 100, 140, 180, 220];

    for cycle in 1..=220 {
        cycles_for_current_command += 1;

        if cycles_to_look_at.contains(&cycle) {
            signal_strength += cycle * x;
        }

        if cycles_for_current_command >= command.cycles_to_complete() {
            command.apply(&mut x);
            command = commands.next().unwrap();
            cycles_for_current_command = 0;
        }
    }

    signal_strength
}

fn ex2(input: String) -> String {
    let mut commands = to_commands(input.lines());

    let mut x = 1;
    let mut command = commands.next().unwrap();
    let mut cycles_for_current_command = 0;

    let mut output = String::new();

    for _ in 0..6 {
        for pixel in 0..40 {
            cycles_for_current_command += 1;

            output.push(if pixel >= x - 1 && pixel <= x + 1 {
                '#'
            } else {
                '.'
            });

            if cycles_for_current_command >= command.cycles_to_complete() {
                command.apply(&mut x);
                command = commands.next().unwrap_or(Noop);
                cycles_for_current_command = 0;
            }
        }
        output.push('\n');
    }

    output
}

fn to_commands(lines: Lines) -> impl Iterator<Item = Command> + '_ {
    let add_regex = Regex::new(r"addx (-?\d+)").unwrap();

    lines.map(move |line| {
        if line == "noop" {
            Noop
        } else if let Some(captures) = add_regex.captures(line) {
            AddX(captures[1].parse().unwrap())
        } else {
            panic!("{line}")
        }
    })
}

tests!(13140, "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n".to_string());
