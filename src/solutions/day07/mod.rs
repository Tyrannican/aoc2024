use super::Solve;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Default, Debug)]
struct Equation {
    target: u64,
    operators: Vec<u64>,
}

impl Equation {
    pub fn can_be_true(&self) -> bool {
        is_valid(self.target, 0, &self.operators)
    }

    pub fn can_be_true_with_concatentation(&self) -> bool {
        is_valid_with_concatenation(self.target, 0, &self.operators)
    }
}

pub fn is_valid(target: u64, result: u64, ops: &[u64]) -> bool {
    if ops.is_empty() {
        return result == target;
    }

    let mult = is_valid(target, result * ops[0], &ops[1..]);
    let add = is_valid(target, result + ops[0], &ops[1..]);

    return mult || add;
}

pub fn is_valid_with_concatenation(target: u64, result: u64, ops: &[u64]) -> bool {
    if ops.is_empty() {
        return result == target;
    }

    let mult = is_valid_with_concatenation(target, result * ops[0], &ops[1..]);
    let add = is_valid_with_concatenation(target, result + ops[0], &ops[1..]);
    let concat_op = format!("{result}{}", ops[0]).parse::<u64>().unwrap();
    let concat = is_valid_with_concatenation(target, concat_op, &ops[1..]);

    mult || add || concat
}

#[derive(Default)]
pub struct Solution {
    equations: Vec<Equation>,
}

impl Solution {
    pub fn new() -> Self {
        let mut sol = Self::default();
        let _ = sol.read_input("./src/solutions/day07/input.txt");
        sol
    }
}

impl Solve for Solution {
    fn read_input(&mut self, path: &str) -> Result<()> {
        self.equations = std::fs::read_to_string(path)?
            .trim()
            .split("\n")
            .map(|line| {
                let split: Vec<&str> = line.split(": ").collect();
                Equation {
                    target: split[0].parse::<u64>().unwrap(),
                    operators: split[1]
                        .split(" ")
                        .map(|op| op.parse::<u64>().unwrap())
                        .collect(),
                }
            })
            .collect();

        Ok(())
    }

    fn part1(&mut self) -> Result<()> {
        let total: u64 = self
            .equations
            .iter()
            .filter_map(|eq| {
                if eq.can_be_true() {
                    return Some(eq.target);
                }

                None
            })
            .sum();

        println!("Total: {total}");

        Ok(())
    }

    fn part2(&mut self) -> Result<()> {
        let total: u64 = self
            .equations
            .iter()
            .filter_map(|eq| {
                if eq.can_be_true_with_concatentation() {
                    return Some(eq.target);
                }

                None
            })
            .sum();

        println!("Total: {total}");
        Ok(())
    }
}
