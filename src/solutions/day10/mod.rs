use std::collections::HashSet;

use super::Solve;
use anyhow::Result;

type Pos = (isize, isize);

#[derive(Default)]
pub struct Solution {
    map: Vec<Vec<usize>>,
    starts: Vec<Pos>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day10/input.txt");
        sol
    }

    fn find_path(&self, pos: Pos) -> (HashSet<Pos>, usize) {
        let mut total = 0;
        let mut path = HashSet::new();
        if let Some(current) = self.get_position(pos) {
            if current == 9 {
                path.insert(pos);
                total += 1;
                return (path, total);
            }

            let next_steps = self.get_next_step(pos);
            for next_step in next_steps {
                let (other_path, other) = self.find_path(next_step);
                total += other;
                path.extend(other_path);
            }
        }

        (path, total)
    }

    fn get_next_step(&self, pos: Pos) -> Vec<Pos> {
        let cardinals = self.get_cardinals(pos);
        cardinals
            .iter()
            .filter_map(|&c| {
                if let Some(next) = self.get_position(c) {
                    if self.get_position(pos).unwrap() + 1 == next {
                        return Some(c);
                    }
                }

                None
            })
            .collect()
    }

    fn get_cardinals(&self, pos: Pos) -> [Pos; 4] {
        [
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ]
    }

    fn get_position(&self, pos: Pos) -> Option<usize> {
        if (pos.0 >= 0 && pos.0 < self.map.len() as isize)
            && (pos.1 >= 0 && pos.1 < self.map[0].len() as isize)
        {
            return Some(self.map[pos.0 as usize][pos.1 as usize]);
        }

        None
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.map = std::fs::read_to_string(path)?
            .trim()
            .split("\n")
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, ch)| {
                        let height = ch.to_digit(10).unwrap() as usize;
                        if height == 0 {
                            self.starts.push((x as isize, y as isize));
                        }

                        height
                    })
                    .collect()
            })
            .collect();
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut total = 0;
        for start in &self.starts {
            let (path, _) = self.find_path(*start);
            total += path.len();
        }

        println!("Part 1: {total}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut total = 0;
        for start in &self.starts {
            let (_, d_total) = self.find_path(*start);
            total += d_total;
        }

        println!("Part 2: {total}");
        Ok(())
    }
}
