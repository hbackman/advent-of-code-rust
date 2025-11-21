use regex::Regex;

pub fn part1(input: &str) -> i32 {
    Regex::new(r"mul\((-?\d+),(-?\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|caps| {
            let a: i32 = caps[1].parse().unwrap();
            let b: i32 = caps[2].parse().unwrap();
            a * b
        })
        .sum()
}

pub fn part2(input: &str) -> i32 {
    Regex::new(r"(mul|do|don't)\((?:(-?\d+),(-?\d+))?\)")
        .unwrap()
        .captures_iter(input)
        .fold((true, 0), |(enabled, sum), caps| {
            match &caps[1] {
                "mul" if enabled => {
                    if let (Some(a), Some(b)) = (caps.get(2), caps.get(3)) {
                        let a: i32 = a.as_str().parse().unwrap();
                        let b: i32 = b.as_str().parse().unwrap();
                        (enabled, sum + a * b)
                    } else {
                        (enabled, sum)
                    }
               }
               "mul" => (enabled, sum), // disabled
               "do" => (true, sum),
               "don't" => (false, sum),
               _ => (enabled, sum),
           }
        })
        .1
}
