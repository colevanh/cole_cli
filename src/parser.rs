/// Finds and returns the nth_word in a string slice
/// Isolates the white space before and after the sought after word
pub fn nth_word(my_str: &str, x: u32) -> &str {

    let my_bytes = my_str.as_bytes();
    let mut counter: u32 = 0;

    let mut starting_index = 0;
    let mut ending_index = 0;

    for (i, &item) in my_bytes.iter().enumerate() {
        if item == b' ' {
            counter += 1;
        

            if counter == x - 1 {
                starting_index = i + 1;
            }

            if counter == x {
                ending_index = i;
                return &my_str[starting_index..ending_index]
            }
        }
    }

    &my_str[starting_index..]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING_1: &str = "Hey Tamra Get Lost";
    const TEST_STRING_2: &str = "hello";
    const TEST_STRING_3: &str = "a f d f sd 3dd fd d3 f sdd s";
    const TEST_STRING_EMPTY: &str = "";

    #[test]
    fn test_nth_word() {
        assert_eq!(nth_word(TEST_STRING_1, 2), "Tamra");
        assert_eq!(nth_word(TEST_STRING_2, 1), "hello");
        assert_eq!(nth_word(TEST_STRING_3, 6), "3dd");
        assert_eq!(nth_word(TEST_STRING_3, 30), "a f d f sd 3dd fd d3 f sdd s");
        assert_eq!(nth_word(TEST_STRING_EMPTY, 1), "");

    }

}


