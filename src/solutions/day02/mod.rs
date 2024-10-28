use super::Solve;
use anyhow::Result;

pub struct Solution;

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self;
        sol.read_input("./src/day02/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) {}

    fn part1(&mut self) -> Result<()> {
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        Ok(())
    }
}
