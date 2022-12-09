use std::collections::HashSet;

use crate::tests;

fn ex1(input: String) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();

    for (direction, steps) in input.lines().map(split_line) {
        for _ in 0..steps {
            head = move_in(direction, head);
            tail = follow(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn ex2(input: String) -> usize {
    let mut snake = vec![(0, 0); 10];
    let mut visited = HashSet::new();

    for (direction, steps) in input.lines().map(split_line) {
        for _ in 0..steps {
            let head = snake.get_mut(0).unwrap();
            *head = move_in(direction, *head);

            for i in 1..=9 {
                let previous = snake[i - 1];
                let current = snake.get_mut(i).unwrap();
                *current = follow(previous, *current);
            }
            visited.insert(snake[9]);
        }
    }

    visited.len()
}

fn split_line(line: &str) -> (char, usize) {
    let mut split = line.split(" ");
    (
        split.next().unwrap().chars().next().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

fn move_in(direction: char, (x, y): (i32, i32)) -> (i32, i32) {
    match direction {
        'R' => (x + 1, y),
        'L' => (x - 1, y),
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        _ => panic!(),
    }
}

fn follow((head_x, head_y): (i32, i32), (tail_x, tail_y): (i32, i32)) -> (i32, i32) {
    let dx = head_x - tail_x;
    let dy = head_y - tail_y;

    if dx.abs() < 2 && dy.abs() < 2 {
        return (tail_x, tail_y);
    }

    if dy == 0 {
        (tail_x + dx.signum(), tail_y)
    } else if dx == 0 {
        (tail_x, tail_y + dy.signum())
    } else {
        (tail_x + dx.signum(), tail_y + dy.signum())
    }
}

tests!(88, 36);
