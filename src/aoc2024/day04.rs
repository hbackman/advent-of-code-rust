fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn find_xmas_maybes(grid: &Vec<Vec<char>>, target: char) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == target {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn find_xmas_strings(grid: &Vec<Vec<char>>, positions: &[(usize, usize)]) -> usize {
    let directions = [
        // linear
        [(0, 0), ( 1,  0), ( 2,  0), ( 3,  0)],
        [(0, 0), (-1,  0), (-2,  0), (-3,  0)],
        [(0, 0), ( 0,  1), ( 0,  2), ( 0,  3)],
        [(0, 0), ( 0, -1), ( 0, -2), ( 0, -3)],
        // diagonal
        [(0, 0), ( 1,  1), ( 2,  2), ( 3,  3)],
        [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        [(0, 0), ( 1, -1), ( 2, -2), ( 3, -3)],
        [(0, 0), (-1,  1), (-2,  2), (-3,  3)],
    ];

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    positions.iter().map(|&(x, y)| {
        directions.iter().filter(|direction| {
            let word: String = direction.iter().filter_map(|&(dx, dy)| {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                    Some(grid[ny as usize][nx as usize])
                } else {
                    None
                }
            }).collect();

            word == "XMAS"
        }).count()
    }).sum()
}

fn find_x_mas_strings(grid: &Vec<Vec<char>>, positions: &[(usize, usize)]) -> usize {
    let directions = [
        // M.S    S.M    M.M    S.S
        // .A. -> .A. -> .A. -> .A.
        // M.S    S.M    S.S    M.M
        [(-1, -1), (0, 0), ( 1,  1), (-1,  1), (0, 0), ( 1, -1)],
        [( 1,  1), (0, 0), (-1, -1), ( 1, -1), (0, 0), (-1,  1)],
        [(-1, -1), (0, 0), ( 1,  1), ( 1, -1), (0, 0), (-1,  1)],
        [(-1,  1), (0, 0), ( 1, -1), ( 1,  1), (0, 0), (-1, -1)],
    ];

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    positions.iter().map(|&(x, y)| {
        directions.iter().filter(|direction| {
            let word: String = direction.iter().filter_map(|&(dx, dy)| {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                    Some(grid[ny as usize][nx as usize])
                } else {
                    None
                }
            }).collect();

            word == "MASMAS"
        }).count()
    }).sum()
}

pub fn part1(input: &str) -> usize {
    let g = parse(input);
    let p = find_xmas_maybes(&g, 'X');
    let s = find_xmas_strings(&g, &p);
    s
}

pub fn part2(input: &str) -> usize {
    let g = parse(input);
    let p = find_xmas_maybes(&g, 'A');
    let s = find_x_mas_strings(&g, &p);
    s
}
