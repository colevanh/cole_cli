//NOTE eventually this file will strictly be for parsing a file, not necessarily making operations on it
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use anyhow::{anyhow, Result};
use serde;

#[derive(serde::Deserialize)]
pub struct Config {
    //input_file: File,
    //config_file: File,
    //root_dir: Path,
}


impl Config {
    pub fn load_from_file(path: &str) -> Result<Config> {
        let file_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&file_content).expect("crap"); //todo remove "crap"
        Ok(config)
    }

    pub fn default() -> Config {
        // Return sensible defaults
        Config { /* defaults */}
    }
}

// ? better to use Paths or Strings for reading a file from cold start
// ? I imagine it would be &str
// Wrapper for read_to_string 

#[allow(dead_code)]
pub fn get_file_as_string(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?,)
}

// Helper method for opening a file via Result propagation
#[allow(dead_code)]
pub fn open_file_or_error(file_name: &str) -> Result<File> {
    let file_path = Path::new(file_name);
    let my_file = File::open(file_path)?;
    Ok(my_file)
}


fn get_test_file(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

// Count the lines in a file 
#[allow(dead_code)]
pub fn get_num_lines(file: File) -> Result<u32> {
    let mut my_bufreader = BufReader::new(file);
    let mut num_lines: u32 = 0;
    let mut line = String::new();

    loop {
        let line_bytes = my_bufreader.read_line(&mut line)?;
        println!("{line_bytes} {line}");
        if line_bytes == 0 {
           
            break;
        }
        num_lines += 1;
        line.clear();
    }
    
    Ok(num_lines)
}

// TODO fix it
#[allow(dead_code)]
pub fn print_lines_with_nums(file: File) {
    if let Err(err) = print_lines_with_nums_to_writer(file, &mut io::stdout()) {
        eprintln!("failed to print lines: {err}");
    }
}

pub fn print_lines_with_nums_to_writer<W: Write>(file: File, writer: &mut W) -> Result<()> {
    let my_bufreader = BufReader::new(file);
    let mut bufreader_iterator = my_bufreader.lines().enumerate().peekable();

    while let Some(line_result) = bufreader_iterator.next() {
        let line = match line_result {
            (_, Ok(good_line)) => good_line,
            (_, Err(_)) => return Err(anyhow!("failed to read a line from the input file")),
        };

        let rendered = line.replace('"', "").trim().to_string();

        if bufreader_iterator.peek().is_some() {
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


#[cfg(test)]
mod tests { 
    use super::*;

    // text files for testing
    const TEST_FILE_1: &str = "./tests/text_files/test_file_1.txt";
    const TEST_FILE_2: &str = "./tests/text_files/test_file_2.txt";
    const TEST_FILE_3: &str = "./tests/text_files/test_file_3.txt";
    
    #[test]
    fn test_print_lines_with_nums() {
        let file_1 = File::open(TEST_FILE_2).unwrap();

        print_lines_with_nums(file_1);
    }

    #[test]
    fn test_num_lines() {
        // we expect these hardcoded paths to work, hence the unwrap call
        let file_1 = File::open(TEST_FILE_1).unwrap();
        let file_2 = File::open(TEST_FILE_2).unwrap();
        let file_3 = File::open(TEST_FILE_3).unwrap();
        
        // assert the line count values
        // ? how to avoid unwrapping here? would need to use a match, right
        //assert_eq!(get_num_lines(file_1).unwrap(), 0);
        assert_eq!(get_num_lines(file_2).unwrap(), 4);
        assert_eq!(get_num_lines(file_3).unwrap(), 11);
        
        
    }
}
