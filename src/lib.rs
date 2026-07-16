pub mod cli;
pub mod commands;
pub mod models;
pub mod parser;
pub mod services;

use crate::cli::args::{Cli, Commands};
use clap::Parser;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Print(input_args) => {
            crate::commands::print::run(input_args.input.as_ref().unwrap());
        }
        Commands::NumLines(numlines_args) => {
            crate::commands::num_lines::run(numlines_args.input.as_ref().unwrap());
        }
        Commands::EnvVars(env_args) => {
            crate::commands::dir_info::run()?;
        }
    };

    Ok(())
}
