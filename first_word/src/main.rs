use std::io;

fn main() {

    // Reference from the Ch-4 from the official documentation
    /*
    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("word = {}",word);
    
    let my_string_literal = "hello world";
    
    let word = first_word(my_string_literal);
    println!("first word in my_string_literal = {}",word);

    let length: usize = word.len();

    println!("length of first word = {}",length);
    */

    // Added input from user for testing
    /*
    println!("Enter the string");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let word = first_word(&input);

    println!("First word of the given input : {}",word);
    */

    // Enhanced implementation to find the first word, handling user input and edge cases like empty strings or leading/trailing whitespace.
    println!("Enter the string");

    let mut input_string:String = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read the line");

    // Trim trailing whitespace to ensure clean processing of user input.
    let trimmed_input_string = input_string.trim();

    let word = first_word(trimmed_input_string);

    // handling the case if empty input is given by the user
    if word.is_empty() {
        println!("No words found in the input");
    }
    else {
        println!("First word of the given input: {}",word);
    }

}

// Basic implementation of the first_word function for reference.
/*
fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
*/

// more user-friendly version of first word function

fn first_word(input_string: &str) -> &str {

    const SPACE: u8 = b' '; // Declaring a constant for the space character.

    let bytes = input_string.as_bytes(); // converting the given input as bytes

    for (i, &item) in bytes.iter().enumerate() { // Convert the input string to bytes for efficient traversal.

        if item == SPACE {  // If a space is found, return the first word as slice of the input string
            return &input_string[0..i];
        }
    }

    &input_string[..] // If no space is found, return the entire input string as it contains only one word.
}