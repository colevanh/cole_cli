//NOTE eventually this file will strictly be for parsing a file, not necessarily making operations on it

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Error, Result, Write, stdout};
use std::path::Path;


// Wrapper for read_to_string
#[allow(dead_code)]
pub fn get_file_as_string(path: &Path) -> Result<String> {
    fs::read_to_string(path)
}

// Helper method for opening a file via Result propagation
#[allow(dead_code)]
pub fn open_file_or_error(file_name: &str) -> Result<File> {
    let file_path = Path::new(file_name);
    let my_file = File::open(file_path)?;
    Ok(my_file)
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

// TODO
// Got distracted when trying to make tests worked. Come back and make this function
// like it did originally
#[allow(dead_code)]
pub fn print_lines_with_nums(file: File) {
    let my_bufreader = BufReader::new(file);
    let mut bufreader_iterator = my_bufreader.lines().enumerate().peekable();
    
    while let Some(line_result) = bufreader_iterator.next() {
        let line = match line_result {
            (num, Ok(good_line)) => (num+1, good_line),
            (_, Err(_e)) => panic!("bummer bro"),
        };
        
        if !line.1.is_empty() {
            let new_string = line.1.replace('"', "");
                match bufreader_iterator.peek() {
                    Some(_) =>  println!("{}", new_string.trim()),
                    None => print!("{}", new_string.trim()),
                };
            io::stdout().flush();
        } else {
            match bufreader_iterator.peek() {
                Some(_) =>  println!(""),
                None => print!(""),
            };
            io::stdout().flush();
        }
        
    }   

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
