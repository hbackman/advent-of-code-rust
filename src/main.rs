use std::env;
use std::fs;

mod aoc2024;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <year> <day>");
        eprintln!("Example: cargo run 2024 01");
        return;
    }

    let y: u32 = args[1].parse().expect("Invalid year");
    let d: u32 = args[2].parse().expect("Invalid day");

    // Read input file
    let input_path = format!("input/{}/day{:02}.txt", y, d);
    let input = fs::read_to_string(&input_path)
        .unwrap_or_else(|_| {
            eprintln!("Warning: Could not read input file: {}", input_path);
            String::new()
        });

    match y {
        2024 => aoc2024::solve(d, &input),
        _ => eprintln!("Year {} not implemented", y),
    }
}
