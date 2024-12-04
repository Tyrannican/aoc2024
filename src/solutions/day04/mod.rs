use super::Solve;
use anyhow::Result;

const XMAS: &[char] = &['X', 'M', 'A', 'S'];
const SAMX: &[char] = &['S', 'A', 'M', 'X'];
const MAS: &[char] = &['M', 'A', 'S'];
const SAM: &[char] = &['S', 'A', 'M'];

#[derive(Default)]
pub struct Solution {
    program: Vec<Vec<char>>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day04/input.txt");
        sol
    }

    pub fn horizontal(&self, program: &Vec<Vec<char>>) -> u32 {
        let mut total = 0;

        let mut start = 0;
        let mut end = 4;

        for row in program.iter() {
            while end <= row.len() {
                let check = &row[start..end];
                if check == XMAS || check == SAMX {
                    total += 1;
                }
                start += 1;
                end += 1;
            }

            start = 0;
            end = 4;
        }

        total
    }

    pub fn vertical(&self) -> u32 {
        let mut total = 0;

        let mut start = 0;
        let mut end = 4;

        for col in 0..self.program[0].len() {
            while end <= self.program.len() {
                let s = (start..end)
                    .into_iter()
                    .map(|i| self.program[i][col])
                    .collect::<Vec<char>>();

                if &s[..] == XMAS || &s[..] == SAMX {
                    total += 1;
                }

                start += 1;
                end += 1;
            }

            start = 0;
            end = 4;
        }

        total
    }

    pub fn diagonals(&self) -> u32 {
        let rows = self.program.len();
        let cols = self.program[0].len();

        let mut left_diagonal = vec![];
        for d in 0..(rows + cols - 1) {
            let mut diag = vec![];
            for i in 0..rows {
                let j = d as isize - i as isize;
                if j >= 0 && j < cols as isize {
                    diag.push(self.program[i][j as usize]);
                }
            }

            left_diagonal.push(diag)
        }

        let mut right_diagonal = vec![];

        for d in 0..(rows + cols - 1) {
            let mut diag = vec![];
            for i in 0..rows {
                let j = i as isize + d as isize - (cols as isize - 1);
                if j >= 0 && j < cols as isize {
                    diag.push(self.program[i][j as usize]);
                }
            }

            right_diagonal.push(diag);
        }

        self.horizontal(&left_diagonal) + self.horizontal(&right_diagonal)
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.program = std::fs::read_to_string(path)?
            .trim()
            .split("\n")
            .map(|r| r.chars().collect::<Vec<char>>())
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let mut total = 0;
        total += self.horizontal(&self.program);
        total += self.vertical();
        total += self.diagonals();

        println!("Part 1: {total:?}");
        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let row = 1;
        let col = 1;

        let mut total = 0;
        for row in 1..(self.program.len() - 1) {
            for col in 1..(self.program[0].len() - 1) {
                let center = self.program[row][col];
                if center != 'A' {
                    continue;
                }

                let top_left = self.program[row - 1][col - 1];
                let top_right = self.program[row - 1][col + 1];
                let bottom_left = self.program[row + 1][col - 1];
                let bottom_right = self.program[row + 1][col + 1];

                let left = [top_left, center, bottom_right];
                let right = [top_right, center, bottom_left];

                if (left == SAM || left == MAS) && (right == SAM || right == MAS) {
                    total += 1
                }
            }
        }

        println!("Part 2: {total}");

        Ok(())
    }
}
