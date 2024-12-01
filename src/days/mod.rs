pub mod day1;
// Add other challenges as they are implemented

pub fn run_challenge(day: usize) {
    match day {
        1 => day1::run(),
        // Add more matches as needed
        _ => println!("Challenge for day {} is not implemented yet!", day),
    }
}