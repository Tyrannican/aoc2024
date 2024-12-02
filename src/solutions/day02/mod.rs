use super::Solve;
use anyhow::Result;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
        let reports: Vec<_> = reader
            .lines()
            .map(|line| {
                let line = line.expect("should be a valid line");
                line.split(" ")
                    .map(|item| item.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect();

        println!("Reports: {:?}", reports);

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        Ok(())
    }
}
