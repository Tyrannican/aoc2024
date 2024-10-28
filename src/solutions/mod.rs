use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub trait Solve {
    fn read_input(&mut self, path: &str);
    fn part1(&mut self) -> Result<()>;
    fn part2(&mut self) -> Result<()>;
}

pub fn load_solution(day: u8) -> Result<Box<dyn Solve>> {
    match day {
        1 => Ok(Box::new(day01::Solution::new())),
        2 => Ok(Box::new(day02::Solution::new())),
        3 => Ok(Box::new(day03::Solution::new())),
        4 => Ok(Box::new(day04::Solution::new())),
        5 => Ok(Box::new(day05::Solution::new())),
        6 => Ok(Box::new(day06::Solution::new())),
        7 => Ok(Box::new(day07::Solution::new())),
        8 => Ok(Box::new(day08::Solution::new())),
        9 => Ok(Box::new(day09::Solution::new())),
        10 => Ok(Box::new(day10::Solution::new())),
        11 => Ok(Box::new(day11::Solution::new())),
        12 => Ok(Box::new(day12::Solution::new())),
        13 => Ok(Box::new(day13::Solution::new())),
        14 => Ok(Box::new(day14::Solution::new())),
        15 => Ok(Box::new(day15::Solution::new())),
        16 => Ok(Box::new(day16::Solution::new())),
        17 => Ok(Box::new(day17::Solution::new())),
        18 => Ok(Box::new(day18::Solution::new())),
        19 => Ok(Box::new(day19::Solution::new())),
        20 => Ok(Box::new(day20::Solution::new())),
        21 => Ok(Box::new(day21::Solution::new())),
        22 => Ok(Box::new(day22::Solution::new())),
        23 => Ok(Box::new(day23::Solution::new())),
        24 => Ok(Box::new(day24::Solution::new())),
        25 => Ok(Box::new(day25::Solution::new())),
        _ => anyhow::bail!(format!("Invalid AoC day supplied: {day}")),
    }
}
