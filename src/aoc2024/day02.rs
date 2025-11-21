fn parse(line: &str) -> Vec<u8> {
    line.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn check(values: Vec<u8>) -> bool {
    let mut inc = true;
    let mut dec = true;

    for win in values.windows(2) {
        if win[0].abs_diff(win[1]) > 3 {
            return false;
        }

        inc &= win[0] < win[1];
        dec &= win[0] > win[1];

        if !(inc ^ dec) {
            return false;
        }
    }

    true
}

fn check_with_dampner(values: Vec<u8>) -> bool {
    if check(values.clone()) {
        return true;
    }

    for i in 0..values.len() {
        let mut modified = values.clone();
        modified.remove(i);
        if check(modified) {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| check(parse(&l)))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| check_with_dampner(parse(&l)))
        .count()
}
