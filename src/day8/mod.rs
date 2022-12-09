use std::collections::HashMap;

use crate::tests;

struct Map {
    map: HashMap<(usize, usize), usize>,
    x_max: usize,
    y_max: usize,
}

fn ex1(input: String) -> usize {
    let map = build_map(input);

    let mut count = 0;
    for ((x, y), height) in &map.map {
        if is_visible(x, y, height, &map.map) {
            count += 1;
        };
    }
    count
}

fn ex2(input: String) -> usize {
    let map = build_map(input);

    map.map
        .iter()
        .map(|entry| compute_scenic_score(entry, &map))
        .max()
        .unwrap()
}

fn build_map(input: String) -> Map {
    let map: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(move |(x, h)| ((x + 1, y + 1), h))
        })
        .collect();

    let x_max = *map.keys().map(|(x, _)| x).max().unwrap();
    let y_max = *map.keys().map(|(_, y)| y).max().unwrap();

    Map { map, x_max, y_max }
}

fn is_visible(x: &usize, y: &usize, height: &usize, map: &HashMap<(usize, usize), usize>) -> bool {
    let points_to_look_at = map.iter().filter(|((p, q), _)| p == x || q == y);

    let to_left = points_to_look_at
        .clone()
        .find(|((p, _), h)| p < x && *h >= height);
    if to_left.is_none() {
        return true;
    }
    let to_right = points_to_look_at
        .clone()
        .find(|((p, _), h)| p > x && *h >= height);
    if to_right.is_none() {
        return true;
    }
    let up = points_to_look_at
        .clone()
        .find(|((_, q), h)| q < y && *h >= height);
    if up.is_none() {
        return true;
    }
    let down = points_to_look_at
        .clone()
        .find(|((_, q), h)| q > y && *h >= height);
    if down.is_none() {
        return true;
    }
    false
}

fn compute_scenic_score(((x, y), height): (&(usize, usize), &usize), map: &Map) -> usize {
    if x == &1 || y == &1 {
        return 0;
    }

    let mut left = 0;
    for p in (1..*x).rev() {
        left += 1;
        if &map.map[&(p, *y)] >= height {
            break;
        }
    }
    let mut right = 0;
    for p in (*x + 1)..=map.x_max {
        right += 1;
        if &map.map[&(p, *y)] >= height {
            break;
        }
    }
    let mut up = 0;
    for p in (1..*y).rev() {
        up += 1;
        if &map.map[&(*x, p)] >= height {
            break;
        }
    }
    let mut down = 0;
    for p in (*y + 1)..=map.y_max {
        down += 1;
        if &map.map[&(*x, p)] >= height {
            break;
        }
    }

    left * right * up * down
}

tests!(21, 8);
