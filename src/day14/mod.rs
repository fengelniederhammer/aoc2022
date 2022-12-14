use geo::Coord;
use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;

use crate::tests;

struct OccupiedPoints {
    points: HashSet<Coord<isize>>,
    lowest_y: isize,
}

fn ex1(input: String) -> usize {
    let mut occupied_points = get_occupied_points(input);
    let initial_count = occupied_points.points.len();

    let directions = vec![
        Coord::from((0, 1)),
        Coord::from((-1, 1)),
        Coord::from((1, 1)),
    ];

    let mut current = (500, 0).into();
    loop {
        current = match step(current, &directions, &occupied_points) {
            None => {
                occupied_points.points.insert(current);
                (500, 0).into()
            }
            Some(new) => new,
        };

        if current.y > occupied_points.lowest_y {
            return occupied_points.points.len() - initial_count;
        }
    }
}

fn ex2(input: String) -> usize {
    let mut occupied_points = get_occupied_points(input);
    let initial_count = occupied_points.points.len();

    let directions = vec![
        Coord::from((0, 1)),
        Coord::from((-1, 1)),
        Coord::from((1, 1)),
    ];

    let mut current = (500, 0).into();
    loop {
        current = match step(current, &directions, &occupied_points) {
            None => {
                if current == (500, 0).into() {
                    return occupied_points.points.len() - initial_count + 1;
                }

                occupied_points.points.insert(current);
                (500, 0).into()
            }
            Some(new) => new,
        };
    }
}

fn get_occupied_points(input: String) -> OccupiedPoints {
    let lines: HashSet<_> = input
        .lines()
        .map(to_points)
        .flat_map(|points| points.into_iter())
        .collect();
    let lowest_y = lines.iter().map(|line| line.y).max().unwrap();
    OccupiedPoints {
        points: lines,
        lowest_y,
    }
}

fn to_points(line: &str) -> HashSet<Coord<isize>> {
    let regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)(?P<rest>.*)?").unwrap();

    let mut points = vec![];
    let mut to_parse = line.to_string();
    while let Some(captures) = regex.captures(&to_parse) {
        let x: isize = captures["x"].parse().unwrap();
        let y: isize = captures["y"].parse().unwrap();
        points.push((x, y));

        to_parse = captures["rest"].to_string();
    }

    let mut set = HashSet::new();
    for window in points.windows(2) {
        let mut from = Coord::from(window[0]);
        let to = Coord::from(window[1]);

        let delta = to - from;
        let step = Coord::from((delta.x.signum(), delta.y.signum()));

        set.insert(from);
        for _ in 0..max(delta.x.abs(), delta.y.abs()) {
            from = from + step;
            set.insert(from);
        }
    }

    set
}

fn step(
    current: Coord<isize>,
    directions: &[Coord<isize>],
    occupied_points: &OccupiedPoints,
) -> Option<Coord<isize>> {
    for step in directions {
        let new = current + *step;
        if !occupied_points.points.contains(&new) && new.y < occupied_points.lowest_y + 2 {
            return Some(new);
        }
    }

    None
}

tests!(24, 93);
