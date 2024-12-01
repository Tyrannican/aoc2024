use super::Solve;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct Solution {
    left_id: Vec<u32>,
    right_id: Vec<u32>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        sol.read_input("./src/solutions/day01/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) {
        let _: Vec<_> = std::fs::read_to_string(path)
            .expect("should exist")
            .trim()
            .split("\n")
            .map(|item| {
                let ids = item
                    .split("   ")
                    .map(|id| id.parse::<u32>().expect("input should be valid"))
                    .collect::<Vec<u32>>();
                self.left_id.push(ids[0]);
                self.right_id.push(ids[1]);
            })
            .collect();

        self.left_id.sort();
        self.right_id.sort();
    }

    fn part1(&mut self) -> Result<()> {
        let total = self
            .left_id
            .iter()
            .zip(self.right_id.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>();

        println!("Part 1: {total}");
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let result = self
            .left_id
            .iter()
            .map(|id| id * self.right_id.iter().filter(|r| *r == id).count() as u32)
            .sum::<u32>();

        println!("Part 2: {result}");

        Ok(())
    }
}
