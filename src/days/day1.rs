use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Running challenge for day 1!");

    let input_file = "inputs/day1_input.txt";

    // Open the file
    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file {}: {}", input_file, error);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    // Process each line and split into lists
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(content) => {
                // Split the line into two numbers
                let numbers: Vec<i32> = content
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

                if numbers.len() == 2 {
                    left.push(numbers[0]);
                    right.push(numbers[1]);
                } else {
                    eprintln!("Invalid line format: {}", content);
                }
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
            }
        }
    }

    // Sort
    left.sort();
    right.sort();

    // Walk thru and add to sum
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    // Output the results
    println!("Results: {:?}", sum);
}
