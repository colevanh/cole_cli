use std::fs;
use std::fs::File;
use std::process::Command;
use predicates::prelude::*;
use assert_cmd::prelude::*;
use anyhow::Result;
use pretty_assertions::assert_eq;
use cole_cli::services::file_service::print_lines_to_writer;

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

    let raw_stdout = String::from_utf8(output.stdout)
        .expect("invalid UTF8");
    let clean_stdout = raw_stdout
        .replace("\r\n", "\n");

    assert_eq!(clean_stdout, expected);
    Ok(())
}

#[test]
fn print_lines_one() -> Result<()> {
    run(
        &["print", "--input", "./tests/inputs/text_files/test_file_1.txt"], 
        "./tests/inputs/text_files/test_file_1.txt"
    )
}
/// Fail on Windows, pass on MacOS
#[test]
fn print_lines_two() -> Result<()> {
    run(
        &["print", "--input", "./tests/inputs/text_files/test_file_2.txt"], 
        "./tests/inputs/text_files/test_file_2.txt"
    )
}
/// Fail on Windows, pass on MacOS
#[test]
fn print_lines_three() -> Result<()> {
    run(
        &["print", "--input", "./tests/inputs/text_files/test_file_3.txt"], 
        "./tests/inputs/text_files/test_file_3.txt"
    )
}

#[test]
fn direct_capture_matches_cli_output() -> Result<()> {
    let input_path = "./tests/inputs/text_files/test_file_1.txt";
    let expected = fs::read_to_string(input_path)?;

    let output = Command::cargo_bin("cole_cli")?
        .args(["print", "--input", input_path])
        .output()
        .expect("fail");

    let cli_stdout = String::from_utf8(output.stdout)
        .expect("invalid UTF8")
        .trim_end_matches('\n')
        .to_string();

    let input_file = File::open(input_path)?;
    let mut captured = Vec::new();
    
    print_lines_to_writer(input_file, &mut captured)?;

    let direct_stdout = String::from_utf8(captured)?
        .trim_end_matches('\n')
        .to_string();

    assert_eq!(cli_stdout, direct_stdout);
    assert_eq!(direct_stdout, expected.trim_end_matches('\n'));
    Ok(())
}