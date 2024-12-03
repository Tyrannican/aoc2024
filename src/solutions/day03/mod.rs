use super::Solve;
use anyhow::Result;
use regex::Regex;

#[derive(Default)]
pub struct Solution {
    program: String,
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

    pub fn parse_input(&self) -> Vec<Instruction> {
        let mut iter = self.program.chars();
        let mut instructions = vec![];
        let mut instr = String::new();
        while let Some(ch) = iter.next() {
            match ch {
                'm' => {
                    if instr.is_empty() {
                        instr.push(ch)
                    }
                }
                'u' => {
                    if instr.starts_with("m") {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                'l' => {
                    if instr.starts_with("mu") {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                '(' => {
                    if instr == "mul" || instr == "do" || instr == "don't" {
                        instr.push(ch)
                    } else {
                        instr.clear();
                    }
                }
                '0'..='9' | ',' => {
                    if instr.starts_with("mul(") {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                ')' => {
                    if instr.starts_with("mul(") || &instr == "do(" || &instr == "don't(" {
                        instr.push(ch);
                    }
                    if instr.starts_with("mul(") {
                        instructions.push(Instruction::Instr(instr.clone()));
                    } else if &instr == "do()" {
                        instructions.push(Instruction::Enabled);
                    } else if &instr == "don't()" {
                        instructions.push(Instruction::Disabled);
                    }

                    instr.clear();
                    continue;
                }
                'd' => {
                    if instr.is_empty() {
                        instr.push(ch);
                    }
                }
                'o' => {
                    if instr == "d" {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                '\'' => {
                    if instr == "don" {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                'n' => {
                    if instr == "do" {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                't' => {
                    if instr == "don'" {
                        instr.push(ch);
                    } else {
                        instr.clear();
                    }
                }
                _ => instr.clear(),
            }
        }

        instructions
    }
}

#[derive(Debug)]
pub enum Instruction {
    Instr(String),
    Enabled,
    Disabled,
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
        let instructions = self.parse_input();
        let mut enabled = true;
        let result = instructions
            .iter()
            .filter_map(|i| {
                let mut res = 0;
                match i {
                    Instruction::Instr(mul) => {
                        let mul = mul.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
                        println!("Mul: {mul}");
                        res = mul
                            .split(",")
                            .map(|v| v.parse::<u64>().unwrap())
                            .product::<u64>();
                    }
                    Instruction::Enabled => enabled = true,
                    Instruction::Disabled => enabled = false,
                }

                if enabled {
                    return Some(res);
                }

                None
            })
            .sum::<u64>();

        println!("Part 2: {result}");
        Ok(())
    }
}
