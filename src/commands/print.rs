use std::fs::File;
use std::io;

use crate::services::file_service::{open_file_or_error, print_lines_to_writer, print_lines_with_nums_to_writer};

/// top level function called when user chooses the 'print' command
pub fn run(input: &str, line_nums: bool) -> Result<(), Box<dyn std::error::Error>> {
    let file = open_file_or_error(input)?;
    if line_nums {
        print_with_nums(file, &mut io::stdout())?;
    } else {
        print_lines(file, &mut io::stdout())?;
    }
    Ok(())
}

/// function called prints lines to the writer
pub fn print_with_nums(file: File, writer: &mut dyn std::io::Write) -> Result<(), anyhow::Error> {
    print_lines_with_nums_to_writer(file, writer)
}

pub fn print_lines(file: File, writer: &mut dyn std::io::Write) -> Result<(), anyhow::Error> {
    print_lines_to_writer(file, writer)
}