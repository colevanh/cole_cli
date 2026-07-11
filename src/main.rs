use clap::{Parser, Subcommand};
// use std::{fs::File, path::Path};
// use std::env::current_dir;
mod parser;

use parser::{*};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input", default_value = "-")]
    input: Option<String>,

    #[arg(
        short = 'n', 
        long = "name", 
        help = "The user's first name"
    )]
    name: String,

    #[arg(
        short = 'e', 
        long = "email", 
        help = "The user's email address"
    )]
    email: String,

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
    let mut input_file_name: &String = &String::new();
    
    if let Some(filename) = &args.input {
        input_file_name = filename;
    } else {
        panic!("terttt");
    }

    let input_file = match open_file_or_error(&input_file_name) {
        Ok(file) => file,
        Err(_e) => panic!("File Not Found!")
    };
    
    match &args.cmd {
        Commands::Get{value} => println!("{value}"),
        Commands::Set{key: _, value: _, is_true: _} => println!("reba"),
        Commands::Print => print_lines_with_nums(input_file),
    }   

    
   
    Ok(())
}