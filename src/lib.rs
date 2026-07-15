pub mod cli;
pub mod commands;
pub mod models;
pub mod parser;
pub mod services;

use crate::cli::args::{Cli, Commands};
use clap::Parser;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let input = cli.input.as_deref().unwrap_or("-");

    match cli.command {
        Commands::Print => crate::commands::print::run(input)?,
        Commands::NumLines => crate::commands::num_lines::run(input)?,
    }

    Ok(())
}
