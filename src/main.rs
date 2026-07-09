mod parser;

use clap::Parser;
use std::path::Path;
use std::env::current_dir;

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input: Option<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    if let Some(filename) = args.input {
        // TODO call a parse_file function here
        println!("tempo");
    } else {
        // let result = process_input()? // TODO make a method for this
        
    }
    
    Ok(())
}