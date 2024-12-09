use super::Solve;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn antinodes(&self, other: &Self) -> [Self; 2] {
        let delta = other.sub(self);
        [self.sub(&delta), other.add(&delta)]
    }
}

#[derive(Default)]
pub struct Solution {
    height: isize,
    width: isize,
    antennas: HashMap<char, Vec<Point>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day08/input.txt");
        sol
    }

    fn in_bounds(&self, pt: &Point) -> bool {
        (pt.x >= 0 && pt.x < self.height) && (pt.y >= 0 && pt.y < self.width)
    }

    fn find_antinodes(&self, pt1: &Point, pt2: &Point) -> [Point; 2] {
        pt1.antinodes(pt2)
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        let input = std::fs::read_to_string(path)?;
        let input = input.trim().split("\n").collect::<Vec<&str>>();

        self.height = input.len() as isize;
        self.width = input[0].len() as isize;

        let _: Vec<_> = input
            .into_iter()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, ch)| {
                        if ch == '.' {
                            return;
                        }

                        self.antennas
                            .entry(ch)
                            .and_modify(|antenna| antenna.push(Point::new(x as isize, y as isize)))
                            .or_insert(vec![Point::new(x as isize, y as isize)]);
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut antinodes = HashSet::new();
        for (key, antennas) in &self.antennas {
            for (idx, pt1) in antennas.iter().enumerate() {
                for pt2 in antennas[idx + 1..].iter() {
                    let anti = self.find_antinodes(pt1, pt2);
                    antinodes.extend(anti);
                }
            }
        }

        println!(
            "Part 1: {}",
            antinodes.iter().filter(|an| self.in_bounds(an)).count()
        );
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        Ok(())
    }
}
