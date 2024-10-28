use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct AocCli {
    #[clap(subcommand)]
    pub command: AocCommand,
}

#[derive(Subcommand, Debug)]
pub enum AocCommand {
    Day { value: u8 },
}
