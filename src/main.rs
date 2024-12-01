mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    days::run_challenge(day);
}
