fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts.next()?.parse().ok()?;
            let b = parts.next()?.parse().ok()?;
            Some((a, b))
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let (mut l, mut r): (Vec<u32>, Vec<u32>) = parse(input)
        .into_iter()
        .unzip();

    l.sort();
    r.sort();

    l.iter().zip(r).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part2(input: &str) -> i32 {
    2
}
