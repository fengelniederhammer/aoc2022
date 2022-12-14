use crate::tests;
use std::cmp::Ordering;
use std::iter;

const A: u8 = 97;
const Z: u8 = 122;

fn ex1(input: String) -> usize {
    let mut map = to_map(input);

    let (start, target) = find_start_and_target(&map);
    *map.get_mut(start.1).unwrap().get_mut(start.0).unwrap() = (A, true);
    *map.get_mut(target.1).unwrap().get_mut(target.0).unwrap() = (Z, false);

    compute_steps_to_climb(map, start, target)
}

fn ex2(input: String) -> usize {
    let mut map = to_map(input);

    let (start, target) = find_start_and_target(&map);
    *map.get_mut(start.1).unwrap().get_mut(start.0).unwrap() = (A, false);
    *map.get_mut(target.1).unwrap().get_mut(target.0).unwrap() = (Z, false);

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, (height, _))| (x, *&y, height))
        })
        .filter(|(_, _, height)| height == &&A)
        .map(|(x, y, _)| compute_steps_to_climb(map.clone(), (x, y), target))
        .min()
        .unwrap()
}

fn to_map(input: String) -> Vec<Vec<(u8, bool)>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .cloned()
                .zip(iter::repeat(false))
                .collect()
        })
        .collect()
}

fn compute_steps_to_climb(
    mut map: Vec<Vec<(u8, bool)>>,
    start: (usize, usize),
    target: (usize, usize),
) -> usize {
    let mut path_endpoints = vec![start];

    for i in 1.. {
        path_endpoints = step(&mut map, path_endpoints);

        if path_endpoints.is_empty() {
            return usize::MAX;
        }
        if path_endpoints.contains(&target) {
            return i;
        }
    }
    panic!();
}

fn find_start_and_target(map: &[Vec<(u8, bool)>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (usize::MAX, usize::MAX);
    let mut target = (usize::MAX, usize::MAX);
    for (y, row) in map.iter().enumerate() {
        for (x, (byte, _)) in row.iter().enumerate() {
            match char::from(*byte) {
                'S' => start = (x, y),
                'E' => target = (x, y),
                _ => {}
            }
            if start.0 < usize::MAX && target.0 < usize::MAX {
                return (start, target);
            }
        }
    }
    panic!();
}

fn step(map: &mut [Vec<(u8, bool)>], path_endpoints: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_path_endpoints = vec![];

    for point in path_endpoints {
        let mut next = get_accessible_neighbours(map, point);
        new_path_endpoints.append(&mut next);
    }

    new_path_endpoints.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        not_equal => not_equal,
    });
    new_path_endpoints.dedup();
    new_path_endpoints
}

fn get_accessible_neighbours(
    map: &mut [Vec<(u8, bool)>],
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    const OFFSETS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut accessible_neighbours = vec![];
    for (dx, dy) in OFFSETS {
        let p = x as isize + dx;
        let q = y as isize + dy;

        if p < 0 || p >= map[0].len() as isize || q < 0 || q >= map.len() as isize {
            continue;
        }

        let p = p as usize;
        let q = q as usize;

        if !map[q][p].1 && map[q][p].0 <= 1 + map[y][x].0 {
            accessible_neighbours.push((p, q));
            map[q][p].1 = true;
        }
    }
    accessible_neighbours
}

tests!(31, 29);
