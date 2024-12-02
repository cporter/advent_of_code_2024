use std::io::{self, BufReader};

use util::read_lines;

fn ascending(xs: &Vec<i32>) -> bool {
    xs.windows(2).all(|pair| {
        pair[0] < pair[1] && (pair[0] - pair[1]).abs() < 4
    })
}

fn descending(xs: &Vec<i32>) -> bool {
    xs.windows(2).all(|pair| {
        pair[0] > pair[1] && (pair[0] - pair[1]).abs() < 4
    })
}

fn allbut(xs: &Vec<i32>, index: usize) -> Vec<i32> {
    xs.iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, x)| *x)
        .collect()
}

fn ascending_drop1(xs: &Vec<i32>) -> bool {
    (0..xs.len()).any(|todrop| ascending(&allbut(xs, todrop)))
}

fn descending_drop1(xs: &Vec<i32>) -> bool {
    (0..xs.len()).any(|todrop| descending(&allbut(xs, todrop)))
}

fn main() {
    let lines = read_lines(BufReader::new(io::stdin().lock()));
    let levels:Vec<Vec<i32>> = lines.into_iter().map(|line|{
        line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>()
    }).collect();
    let part1 = levels.iter().filter(|level| ascending(level) || descending(level)).count();
    let part2 = levels.iter().filter(|level| ascending_drop1(level) || descending_drop1(level)).count();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
