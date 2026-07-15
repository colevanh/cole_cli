use std::fs;
use std::fs::File;
use assert_cmd::Command;
use anyhow::Result;
use pretty_assertions::assert_eq;
use cole_cli::parser::print_lines_with_nums_to_writer;

#[test]
fn works() {
    assert!(true)
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("cole_cli")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout)
        .expect("invalid UTF8").trim_end_matches('\n').to_string();

    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn print_lines_one() -> Result<()> {
    run(
        &["--input", "./tests/text_files/test_file_1.txt", "print"], 
        "./tests/text_files/test_file_1.txt"
    )
}   
#[test]
fn print_lines_two() -> Result<()> {
    run(
        &["--input", "./tests/text_files/test_file_2.txt", "print"], "./tests/text_files/test_file_2.txt"
    )
}

#[test]
fn print_lines_three() -> Result<()> {
    run(
        &["--input", "./tests/text_files/test_file_3.txt", "print"], 
        "./tests/text_files/test_file_3.txt"
    )
}

// TODO Strip out the two blocks of code dealing with handling stdout
// TODO and turn them into helper functions!
// TODO also consider renaming this thing
#[test]
fn direct_capture_matches_cli_output() -> Result<()> {
    let input_path = "./tests/text_files/test_file_3.txt";
    let expected = fs::read_to_string(input_path)?;

    let output = Command::cargo_bin("cole_cli")?
        .args(["--input", input_path, "print"])
        .output()
        .expect("fail");

    let cli_stdout = String::from_utf8(output.stdout)
        .expect("invalid UTF8")
        .trim_end_matches('\n')
        .to_string();

    let input_file = File::open(input_path)?;
    let mut captured = Vec::new();
    
    print_lines_with_nums_to_writer(input_file, &mut captured)?;

    let direct_stdout = String::from_utf8(captured)?
        .trim_end_matches('\n')
        .to_string();

    assert_eq!(cli_stdout, direct_stdout);
    assert_eq!(direct_stdout, expected.trim_end_matches('\n'));
    Ok(())
}