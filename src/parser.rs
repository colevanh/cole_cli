//NOTE eventually this file will strictly be for parsing a file, not necessarily making operations on it

use std::fs::{self, File};
use std::io::{BufRead, BufReader,Result};
use std::path::Path;

#[allow(dead_code)]
pub fn get_file_as_string(path: &Path) -> Result<String> {
    fs::read_to_string(path)
}

#[allow(dead_code)]
pub fn get_num_lines(file: File) -> Result<u32> {
    let mut my_bufreader = BufReader::new(file);
    let mut num_lines: u32 = 0;
    let mut line = String::new();

    loop {
        let line_bytes = my_bufreader.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_lines += 1;
        line.clear();
    }

    Ok(num_lines)
}

#[allow(dead_code)]
pub fn print_lines_with_nums(file: File) {
    let my_bufreader = BufReader::new(file);
    // println!("reached");

    let bufreader_iterator = my_bufreader.lines().enumerate();
    for line_result in bufreader_iterator {
        let line = match line_result {
            (num, Ok(good_line)) => (num+1, good_line),
            (_, Err(_e)) => panic!("bummer bro"),
        };

        if !line.1.is_empty() {
            let new_string = line.1.replace('"', "");
            println!("{:?} {new_string}", line.0);
        } else {
            println!("{:?}", line.0);
        }
    }   
}


#[cfg(test)]
mod tests { 
    use super::*;

    const TEST_FILE_1: &str = "./test_file_1.txt";
    const TEST_FILE_2: &str = "./test_file_2.txt";
    const TEST_FILE_3: &str = "./test_file_3.txt";
    #[test]
    fn test_print_lines_with_nums() {
        let file_1 = File::open(TEST_FILE_3).unwrap();

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
        assert_eq!(get_num_lines(file_1).unwrap(), 0);
        assert_eq!(get_num_lines(file_2).unwrap(), 4);
        assert_eq!(get_num_lines(file_3).unwrap(), 11);
        
        
    }
}




// TODO functions to implement
// Given a file:
// DONE Count the number of lines
//      Print the file (as a String) along with the number lines
// DONE Print each line with the following info next to it char count, word count
//     
