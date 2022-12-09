use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use regex::Regex;

use crate::day7::Command::{CD, LS};
use crate::day7::LsResult::{Dir, File};
use crate::tests;

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<LsResult>),
}

#[derive(Debug)]
enum LsResult {
    File { name: String, size: usize },
    Dir(String),
}

impl From<&str> for LsResult {
    fn from(line: &str) -> Self {
        let file_regex = Regex::new(r"(?P<size>\d+) (?P<file>[\w.]+)").unwrap();
        let dir_regex = Regex::new(r"dir (?P<dir>\w+)").unwrap();

        if let Some(captures) = file_regex.captures(line) {
            return File {
                name: captures["file"].to_string(),
                size: captures["size"].parse().unwrap(),
            };
        } else if let Some(captures) = dir_regex.captures(line) {
            return Dir(captures["dir"].to_string());
        } else {
            panic!()
        }
    }
}

fn ex1(input: String) -> usize {
    let commands = to_commands(input);

    let dirs = build_dir_structure(commands);

    dirs.iter()
        .filter(|(_, dir_type)| *dir_type == &DirType::Dir)
        .map(|(dir, _)| dir)
        .map(|dir| {
            dirs.iter()
                .filter(|(dir_, _)| dir_.to_string().starts_with(&dir.to_string()))
                .filter_map(|(_, dir_type)| match dir_type {
                    DirType::Dir => None,
                    DirType::File(size) => Some(size),
                })
                .sum()
        })
        .filter(|size: &usize| size <= &100000)
        .sum()
}

fn ex2(input: String) -> usize {
    let commands = to_commands(input);

    let dirs = build_dir_structure(commands);

    let total_space: usize = dirs
        .iter()
        .filter_map(|(_, dir_type)| match dir_type {
            DirType::Dir => None,
            DirType::File(size) => Some(size),
        })
        .sum();

    let free_space = 70_000_000 - total_space;

    dirs.iter()
        .filter(|(_, dir_type)| *dir_type == &DirType::Dir)
        .map(|(dir, _)| {
            dirs.iter()
                .filter(|(dir_, _)| dir_.to_string().starts_with(&dir.to_string()))
                .filter_map(|(a, dir_type)| {
                    let size = match dir_type {
                        DirType::Dir => None,
                        DirType::File(size) => Some(size),
                    };
                    size
                })
                .sum()
        })
        .filter(|(size): &(usize)| free_space + *size >= 30_000_000)
        .min()
        .unwrap()
}

fn to_commands(input: String) -> Vec<Command> {
    let mut lines: Vec<_> = input.lines().rev().collect();
    let mut commands = vec![];

    loop {
        match extract_command(lines) {
            None => break,
            Some((command, left_lines)) => {
                commands.push(command);
                lines = left_lines;
            }
        };
    }
    commands
}

fn extract_command(mut lines: Vec<&str>) -> Option<(Command, Vec<&str>)> {
    let cd_regex = Regex::new(r"\$ cd (?P<dir>[\w/.]+)").unwrap();

    let line = lines.pop()?;

    if let Some(captures) = cd_regex.captures(line) {
        Some((CD(captures["dir"].to_string()), lines))
    } else if line == "$ ls" {
        let (ls_lines, left_lines) = extract_ls_lines(lines);
        lines = left_lines;
        Some((LS(ls_lines), lines))
    } else {
        panic!("{line}")
    }
}

fn extract_ls_lines(mut lines: Vec<&str>) -> (Vec<LsResult>, Vec<&str>) {
    let mut ls_lines = vec![];
    loop {
        match lines.pop() {
            None => return (ls_lines, lines),
            Some(ls_line) if ls_line.starts_with("$") => {
                lines.push(ls_line);
                return (ls_lines, lines);
            }
            Some(ls_line) => ls_lines.push(LsResult::from(ls_line)),
        }
    }
}

fn build_dir_structure(commands: Vec<Command>) -> HashMap<Pwd, DirType> {
    let mut dirs = HashMap::new();
    dirs.insert(Pwd::root(), DirType::Dir);
    let mut pwd = Pwd::root();

    for command in commands {
        match command {
            CD(dir) => {
                if &dir == ".." {
                    pwd.pop();
                } else if &dir == "/" {
                    pwd = Pwd::root();
                } else {
                    pwd.push(dir);
                    dirs.insert(pwd.clone(), DirType::Dir);
                }
            }
            LS(ls_results) => {
                for ls_result in ls_results {
                    match ls_result {
                        File { name, size } => {
                            dirs.insert(pwd.with(name), DirType::File(size));
                        }
                        Dir(dir) => {
                            dirs.insert(pwd.with(dir), DirType::Dir);
                        }
                    }
                }
            }
        }
    }

    dirs
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum DirType {
    Dir,
    File(usize),
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Pwd {
    pwd: Vec<String>,
}

impl Pwd {
    fn push(&mut self, dir: String) {
        self.pwd.push(dir);
    }

    fn pop(&mut self) {
        self.pwd.pop();
    }

    fn with(&self, dir: String) -> Self {
        let mut pwd = self.pwd.clone();
        pwd.push(dir);
        Pwd { pwd }
    }

    fn root() -> Self {
        Pwd { pwd: vec![] }
    }
}

impl ToString for Pwd {
    fn to_string(&self) -> String {
        format!("/{}", self.pwd.join("/"))
    }
}

impl Hash for Pwd {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

tests!(95437, 24933642);
