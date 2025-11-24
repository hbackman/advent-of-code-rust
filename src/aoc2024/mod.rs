pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn solve(day: u32, input: &str) {
    match day {
        1 => {
            println!("Day 1 Part 1: {}", day01::part1(input));
            println!("Day 1 Part 2: {}", day01::part2(input));
        }
        2 => {
            println!("Day 2 Part 1: {}", day02::part1(input));
            println!("Day 2 Part 2: {}", day02::part2(input));
        }
        3 => {
            println!("Day 3 Part 1: {}", day03::part1(input));
            println!("Day 3 Part 2: {}", day03::part2(input));
        }
        4 => {
            println!("Day 4 Part 1: {}", day04::part1(input));
            println!("Day 4 Part 2: {}", day04::part2(input));
        }
        _ => eprintln!("Day {} not implemented", day),
    }
}
