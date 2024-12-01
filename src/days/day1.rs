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

    // Walk thru
    let mut sum = 0;
    let mut score = 0;
    for i in 0..left.len() {
        // Add to sum
        sum += (left[i] - right[i]).abs();

        // Count left nums appearance in right
        let count:usize = right.iter().filter(|&n| *n == left[i]).count();
        // Multiply times appeared by leftnum + add to score
        score += left[i] * count as i32;
    }

    // Output the results
    println!("Results: pt1: {:?} pt2: {:?}", sum, score);
}
