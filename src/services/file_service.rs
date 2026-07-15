use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write, ErrorKind};
use std::io::Error as Err;
use std::path::Path;

use anyhow::{anyhow, Result, Error};

pub fn get_file_as_string(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn open_file_or_error(file_name: &str) -> Result<File> {
    let file_path = Path::new(file_name);
    Ok(File::open(file_path)?)
}

pub fn get_num_lines(file: File) -> Result<u32> {
    let mut reader = BufReader::new(file);
    let mut num_lines = 0_u32;
    let mut line = String::new();

    loop {
        let line_bytes = reader.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        line.clear();
    }

    Ok(num_lines)
}

pub fn print_lines_with_nums_to_writer<W: Write + ?Sized>(file: File, writer: &mut W) -> Result<()> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    while let Some(line_result) = lines.next() {
        let line = match line_result {
            Ok(good_line) => good_line,
            Err(_) => return Err(anyhow!("failed to read a line from the input file")),
        };

        let rendered = line.replace('"', "").trim().to_string();

        if lines.peek().is_some() {
            if rendered.is_empty() {
                writeln!(writer)?;
            } else {
                writeln!(writer, "{rendered}")?;
            }
        } else if !rendered.is_empty() {
            write!(writer, "{rendered}")?;
        }
    }

    writer.flush()?;
    Ok(())
}

pub fn print_lines_with_nums(file: File) {
    if let Err(err) = print_lines_with_nums_to_writer(file, &mut io::stdout()) {
        eprintln!("failed to print lines: {err}");
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

use super::*;
    use std::fs::File;

    const TEST_FILE_1: &str = "./tests/text_files/test_file_1.txt";
    const TEST_FILE_2: &str = "./tests/text_files/test_file_2.txt";
    const TEST_FILE_3: &str = "./tests/text_files/test_file_3.txt";


    #[test]
    fn test_open_invalid_file() {
        let invalid_name: &str = "./tests/text_files/invalid_file.txt";
        let file_result = open_file_or_error(invalid_name);

        assert!(file_result.is_err());

        let file_error = file_result.expect_err("tamra");
        let new_error = Err::new(ErrorKind::InvalidFilename, "invalid!");
        //assert_eq!(new_error.kind(), file_error.downcast_mut().);

        //let opened_file = match file_result {
            //Ok(file) => file,
           // Err(err) => panic!("yooo: {err}"),
        //};

    }
    #[test]
    fn test_print_lines_with_nums() {
        let file_2 = File::open(TEST_FILE_2).unwrap();
        print_lines_with_nums(file_2);
    }

    #[test]
    fn test_num_lines() {
        let _file_1 = File::open(TEST_FILE_1).unwrap();
        let file_2 = File::open(TEST_FILE_2).unwrap();
        let file_3 = File::open(TEST_FILE_3).unwrap();

        assert_eq!(get_num_lines(file_2).unwrap(), 4);
        assert_eq!(get_num_lines(file_3).unwrap(), 10);
    }
}