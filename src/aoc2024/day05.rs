fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = input
        .split("\n\n")
        .collect();

    let rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split("|")
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect())
        .collect();

    (rules, updates)
}

fn order(stack: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut sorted = stack.to_vec();
    sorted.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sorted
}

fn middle(stack: &[i32]) -> i32 {
    stack[stack.len() / 2]
}

pub fn part1(input: &str) -> i32 {
    let (rules, stacks) = parse(input);

    stacks
        .iter()
        .filter(|stack| *stack == &order(stack, &rules))
        .map(|stack| middle(stack))
        .sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let (rules, stacks) = parse(input);

    stacks
        .iter()
        .map(|stack| (stack, order(stack, &rules)))
        .filter(|(s1, s2)| s1 != &s2)
        .map(|(_, s2)| s2)
        .map(|stack| middle(&stack))
        .sum::<i32>()
}
