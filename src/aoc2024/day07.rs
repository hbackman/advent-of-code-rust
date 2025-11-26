fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let numbers: Vec<i64> = line
        .split([' ', ':'])
        .filter(|s| !s.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    (numbers[0], numbers[1..].to_vec())
}

fn gen_eq(inputs: &[i64], total: i64, ops: &[&str]) -> Vec<i64> {
    if inputs.is_empty() {
        return vec![total];
    }

    let a = inputs[0];
    let rest = &inputs[1..];

    ops
        .iter()
        .flat_map(|&op| match op {
            "+" => gen_eq(rest, total + a, ops),
            "*" => gen_eq(rest, total * a, ops),
            "||" => {
                let concatenated = format!("{}{}", total, a).parse().unwrap();
                gen_eq(rest, concatenated, ops)
            }
            _ => vec![],
        })
        .collect()
}

fn valid(result: i64, inputs: &[i64], ops: &[&str]) -> bool {
    let results = gen_eq(&inputs[1..], inputs[0], ops);
    results.contains(&result)
}

fn exec(input: &str, ops: &[&str]) -> i64 {
    input
        .lines()
        .map(parse_line)
        .filter(|(result, inputs)| valid(*result, inputs, ops))
        .map(|(result, _)| result)
        .sum()
}

pub fn part1(input: &str) -> i64 {
    exec(input, &["+", "*"])
}

pub fn part2(input: &str) -> i64 {
    exec(input, &["+", "*", "||"])
}
