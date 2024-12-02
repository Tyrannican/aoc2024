use super::Solve;
use anyhow::Result;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn is_safe(levels: &[u8]) -> bool {
    let mut inc = 0;
    let mut dec = 0;

    for pair in levels.windows(2) {
        let (left, right) = (pair[0], pair[1]);
        if left == right {
            return false;
        }

        if left.abs_diff(right) > 3 {
            return false;
        }

        if left > right {
            inc += 1;
        } else {
            dec += 1;
        }
    }

    if inc > 0 && dec > 0 {
        return false;
    }

    true
}

fn can_be_made_safe(levels: &[u8]) -> bool {
    if is_safe(levels) {
        return true;
    }

    let mut idx = 0;
    while idx <= levels.len() - 1 {
        let new_levels = [&levels[..idx], &levels[idx + 1..]].concat();
        if is_safe(&new_levels) {
            return true;
        }
        idx += 1;
    }

    false
}

#[derive(Debug, Default)]
pub struct Solution {
    reports: Vec<Vec<u8>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day02/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        let reader = BufReader::new(File::open(path)?);
        self.reports = reader
            .lines()
            .map(|line| {
                let line = line.expect("should be a valid line");
                let levels = line
                    .split(" ")
                    .map(|item| item.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();

                levels
            })
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result = self.reports.iter().filter(|report| is_safe(report)).count();

        println!("Part 1: {result}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = self
            .reports
            .iter()
            .filter(|report| can_be_made_safe(report))
            .count();

        println!("Part 2: {result}");

        Ok(())
    }
}
