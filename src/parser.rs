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
                println!("Reached:");
                starting_index = i + 1;
            }

            if counter == x {
                ending_index = i;
                println!("starting: {}", starting_index);
                println!("ending: {}", ending_index);
                return &my_str[starting_index..ending_index]
            }
        }
    }
    &my_str[..]
}