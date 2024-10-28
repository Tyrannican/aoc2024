#![allow(unused_variables)] // Cause we have all days here

mod cli;
mod solutions;

use anyhow::Result;
use clap::Parser;
use cli::AocCommand;
use solutions::load_solution;

fn main() -> Result<()> {
    let cli = cli::AocCli::parse();
    let mut solution = match cli.command {
        AocCommand::Day { value } => load_solution(value)?,
    };

    solution.part1()?;
    solution.part2()?;

    Ok(())
}
