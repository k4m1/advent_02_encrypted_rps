use std::collections::HashMap;
use std::fs::File; // Import the `File` type from the `fs` module in the `std` crate
use std::io::{BufReader, BufRead}; // Import the `BufReader` and `BufRead` traits from the `io` module in the `std` crate

fn main() {
    let mut map = HashMap::new();
    map.insert("A X", 4);
    map.insert("A Y", 1);
    map.insert("A Z", 7);
    map.insert("B X", 8);
    map.insert("B Y", 5);
    map.insert("B Z", 2);
    map.insert("C X", 3);
    map.insert("C Y", 9);
    map.insert("C Z", 6);

    let file = match File::open("input.txt") {
        Ok(file) => file, // If the file is successfully opened, store it in the `file` variable
        Err(error) => { // If there was an error opening the file, handle the error
            println!("Error opening file: {}", error); // Print an error message
            return; // Return from the function
        }
    };
    let reader = BufReader::new(file); // Create a new `BufReader` to read the file

    let mut sum = 0; // Declare and initialize a `sum` variable to 0
    let mut sums = Vec::new(); // Declare and initialize an empty `sums` vector to store the sums

     // Loop through each line in the file
     for line in reader.lines() {
        // Read the line into a string
        let line = match line {
            Ok(line) => line, // If the line is successfully read, store it in the `line` variable
            Err(error) => { // If there was an error reading the line, handle the error
                println!("Error reading line: {}", error); // Print an error message
                continue; // Skip to the next iteration of the loop
            }
        };

    map.iter().for_each(|(key, value)| {
        println!("{}: {}", key, value);

    });
    }
}