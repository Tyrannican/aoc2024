use super::Solve;
use anyhow::Result;
use std::collections::HashSet;

type Coord = (usize, usize);

#[derive(Eq, PartialEq, Default, Hash, Copy, Clone, Debug)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn switch(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn next_step(&self, current: Coord) -> Coord {
        let (x, y) = current;
        match self {
            Self::North => (x - 1, y),
            Self::East => (x, y + 1),
            Self::South => (x + 1, y),
            Self::West => (x, y - 1),
        }
    }
}

#[derive(Default)]
pub struct Solution {
    start: Coord,
    map: Vec<Vec<u8>>,
    path: HashSet<Coord>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day06/input.txt");
        sol
    }
}

fn is_boundary(coord: Coord, map: &Vec<Vec<u8>>) -> bool {
    let (x, y) = coord;
    x == 0 || x == map.len() - 1 || y == 0 || y == map[0].len() - 1
}

fn index(coord: Coord, map: &Vec<Vec<u8>>) -> u8 {
    let (x, y) = coord;
    map[x][y]
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
                    .map(|(y, ch)| match ch {
                        '.' => 0,
                        '^' => {
                            self.start = (x, y);
                            0
                        }
                        '#' => 1,
                        _ => unreachable!("no other valid chars present"),
                    })
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut current = self.start;
        let mut dir = Direction::North;

        while !is_boundary(current, &self.map) {
            let next = dir.next_step(current);
            if index(next, &self.map) == 1 {
                dir = dir.switch();
                continue;
            }

            self.path.insert(current);
            current = next;
        }

        self.path.insert(current);

        println!("Part 1: {}", self.path.len());

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let mut cycles = 0;
        for path in self.path.iter() {
            let mut current = self.start;
            let mut dir = Direction::North;

            let (px, py) = *path;
            let mut map = self.map.clone();
            map[px][py] = 1;

            let mut new_path: HashSet<(Coord, Direction)> = HashSet::new();
            while !is_boundary(current, &map) {
                let next = dir.next_step(current);
                if new_path.contains(&(next, dir)) {
                    cycles += 1;
                    break;
                }
                if index(next, &map) == 1 {
                    dir = dir.switch();
                    continue;
                } else {
                    new_path.insert((next, dir));
                    current = next;
                }
            }
        }

        println!("Part 2: {cycles}");

        Ok(())
    }
}
