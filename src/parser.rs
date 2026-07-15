pub use crate::services::file_service::{
    get_file_as_string, get_num_lines, open_file_or_error, print_lines_with_nums,
    print_lines_with_nums_to_writer,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    const TEST_FILE_1: &str = "./tests/text_files/test_file_1.txt";
    const TEST_FILE_2: &str = "./tests/text_files/test_file_2.txt";
    const TEST_FILE_3: &str = "./tests/text_files/test_file_3.txt";

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
