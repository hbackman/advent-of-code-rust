use std::collections::HashSet;
use rayon::prelude::*;

type Grid = Vec<Vec<char>>;
type Pos = (i32, i32, char);

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn start(map: &Grid) -> Option<Pos> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                '^' | 'v' | '<' | '>' => {
                    return Some((x as i32, y as i32, map[y][x]))
                }
                _ => {}
            }
        }
    }
    None
}

fn next(pos: Pos) -> (i32, i32) {
    let (x, y, dir) = pos;
    match dir {
        '^' => (x + 0, y - 1),
        'v' => (x + 0, y + 1),
        '<' => (x - 1, y + 0),
        '>' => (x + 1, y + 0),
        _   => (x + 0, y + 0)
    }
}

fn turn(dir: char) -> char {
    match dir {
        '^' => '>',
        'v' => '<',
        '<' => '^',
        '>' => 'v',
        _   => dir,
    }
}

fn inside(grid: &Grid, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[0].len() as i32
}

enum MoveResult {
    Cont(Pos),
    Halt,
}

fn tick(grid: &Grid, pos: Pos) -> MoveResult {
    let (x, y, dir) = pos;
    let (nx, ny) = next(pos);

    if inside(grid, nx, ny) {
        match grid[ny as usize][nx as usize] {
            '#' => MoveResult::Cont((x, y, turn(dir))),
            _   => MoveResult::Cont((nx, ny, dir)),
        }
    } else {
        MoveResult::Halt
    }
}

enum WalkResult {
    Loop,
    Visited(Vec<Pos>),
}

fn walk(grid: &Grid, pos: Pos, visited: &mut HashSet<Pos>) -> WalkResult {
    if visited.contains(&pos) {
        return WalkResult::Loop;
    }

    visited.insert(pos);

    match tick(grid, pos) {
        MoveResult::Cont(next_pos) => walk(grid, next_pos, visited),
        MoveResult::Halt => WalkResult::Visited(visited.iter().copied().collect()),
    }
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let pos = start(&map).unwrap();

    match walk(&map, pos, &mut HashSet::new()) {
        WalkResult::Loop => 0,
        WalkResult::Visited(path) => {
            let unique_positions: HashSet<(i32, i32)> = path
                .iter()
                .map(|(x, y, _)| (*x, *y))
                .collect();
            unique_positions.len()
        }
    }
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);
    let pos = start(&map).unwrap();

    let path = match walk(&map, pos, &mut HashSet::new()) {
        WalkResult::Visited(p) => p,
        WalkResult::Loop => return 0,
    };

    let positions: HashSet<(i32, i32)> = path
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .filter(|&(x, y)| (x, y) != (pos.0, pos.1))
        .collect();

    positions
         .par_iter()
         .filter(|&&(ox, oy)| {
             let mut test_grid = map.clone();

             test_grid
                 [oy as usize]
                 [ox as usize] = '#';

             let test_path = walk(&test_grid, pos, &mut HashSet::new());

             matches!(test_path, WalkResult::Loop)
         })
         .count()
}
