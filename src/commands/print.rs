use std::fs::File;
use std::io;

use crate::services::file_service::{open_file_or_error, print_lines_with_nums_to_writer};

pub fn run(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = open_file_or_error(input)?;
    print_lines_with_nums(file, &mut io::stdout())?;
    Ok(())
}

pub fn print_lines_with_nums(file: File, writer: &mut dyn std::io::Write) -> Result<(), anyhow::Error> {
    print_lines_with_nums_to_writer(file, writer)
}
