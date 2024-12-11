use std::collections::HashMap;

use super::Solve;
use anyhow::Result;

#[derive(Default)]
pub struct Solution {
    stones: Vec<usize>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day11/input.txt");
        sol
    }

    fn blink(
        &self,
        stone: usize,
        blink_number: usize,
        ceil: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        let mut total = 0;
        let mut new_stone = stone;
        if blink_number == ceil {
            return 1;
        }

        let key = (stone, blink_number);
        if let Some(&value) = cache.get(&key) {
            return value;
        }

        if stone == 0 {
            new_stone = 1;
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                total += self.blink(right, blink_number + 1, ceil, cache);
                new_stone = left;
            } else {
                new_stone *= 2024;
            }
        }

        total += self.blink(new_stone, blink_number + 1, ceil, cache);
        cache.insert((stone, blink_number), total);

        total
    }

    fn run(&mut self, size: usize) -> usize {
        let mut cache = HashMap::new();
        self.stones
            .iter()
            .map(|&s| self.blink(s, 0, size, &mut cache))
            .sum()
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.stones = std::fs::read_to_string(path)?
            .trim()
            .split(" ")
            .map(|stone| stone.parse::<usize>().unwrap())
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let total = self.run(25);
        println!("Total: {total}");
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let total = self.run(75);
        println!("Total: {total}");
        Ok(())
    }
}
