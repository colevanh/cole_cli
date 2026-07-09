use std::fs;
use assert_cmd::Command;
use anyhow::Result;
use pretty_assertions::assert_eq;

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