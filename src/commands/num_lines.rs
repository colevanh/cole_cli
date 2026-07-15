use crate::services::file_service::{get_num_lines, open_file_or_error};

pub fn run(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = open_file_or_error(input)?;
    let num_lines = get_num_lines(file)?;
    println!("Number of lines: {}", num_lines);
    Ok(())
}
