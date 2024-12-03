use super::Solve;
use anyhow::Result;
use regex::Regex;

#[derive(Default)]
pub struct Solution {
    pub program: String,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day03/input.txt");
        sol
    }

    pub fn get_mul_results(&self, input: &str) -> Result<Vec<u64>> {
        let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)")?;
        let result: Vec<u64> = re
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [item])| {
                item.split(",")
                    .map(|d| d.parse::<u64>().unwrap())
                    .product::<u64>()
            })
            .collect();

        Ok(result)
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.program = std::fs::read_to_string(path)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let result: u64 = self.get_mul_results(&self.program)?.iter().sum();
        println!("Part 1: {result}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut enabled = true;
        let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)")?;
        let result: u64 = re
            .find_iter(&self.program)
            .filter_map(|item| {
                let item = item.as_str();
                if item == "do()" {
                    enabled = true;
                    return None;
                } else if item == "don't()" {
                    enabled = false;
                    return None;
                } else {
                    if enabled {
                        return Some(self.get_mul_results(item).unwrap()[0]);
                    }

                    return None;
                }
            })
            .sum();

        println!("Part 2: {result}");

        Ok(())
    }
}
