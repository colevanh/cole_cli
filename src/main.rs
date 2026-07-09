use clap::{Parser, Subcommand};
use std::{fs::File, path::Path};
use std::env::current_dir;
mod parser;

use parser::{*};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        value: String,
    },
    Set {
        key: String,
        value: String,
        is_true: bool
    },
    Print,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let mut input_file_name: String = String::new();
    
    if let Some(filename) = args.input {
        input_file_name = filename;
    } else {
        panic!("terttt");
    }
    
    match args.cmd {
        Commands::Get{value} => println!("{value}"),
        Commands::Set{key, value, is_true} => println!("reba"),
        Commands::Print => print_lines_with_nums(File::open(input_file_name).unwrap()),
    }   

    Ok(())
}