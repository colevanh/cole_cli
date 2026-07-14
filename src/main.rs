use clap::{Parser, Subcommand};
// use std::{fs::File, path::Path};
// use std::env::current_dir;
mod parser;

use parser::{*};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short = 'i', 
        long = "input", 
        default_value = "-",
        help = "file to be used as input"
    )]
    input: Option<String>,

    #[arg(
        short = 'n', 
        long = "name", 
        default_value = "Cole Van H",
        help = "The user's first name"
    )]
    name: String,

    #[arg(
        short = 'e', 
        long = "email", 
        default_value = "colevanh@gmail.com",
        help = "The user's email address"
    )]
    email: String,

    /// Must be ten years old to use this program
    #[arg(
        value_parser = clap::value_parser!(u16).range(10..),
        short,
        long,
    )]

    age: u16,

    #[command(subcommand)]
    cmd: Commands,

}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Print the contents of a file
    Print,
    /// Print the number of lines in a file
    NumLines,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();
    println!("cli args {}", cli.)
    let mut input_file_name: &String = &String::new();
    
    if let Some(filename) = &cli.input {
        input_file_name = filename;
    } else {
        panic!("terttt");
    }

    let input_file = match open_file_or_error(&input_file_name) {
        Ok(file) => file,
        Err(_e) => panic!("File Not Found!")
    };
    
    match &cli.cmd {
        Commands::Print => print_lines_with_nums(input_file),
        Commands::NumLines => {
            let num_lines = get_num_lines(input_file).unwrap();
            println!("Number of lines: {}", num_lines);
        }
    };

    Ok(())
}