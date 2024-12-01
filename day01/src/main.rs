use std::collections::HashMap;
use std::io::{self, BufReader};

use util::read_lines;

fn main() {
    let mut xs:Vec<i32> = vec![];
    let mut ys:Vec<i32> = vec![];
    let mut yc:HashMap<i32, i32> = HashMap::new();

    read_lines(BufReader::new(io::stdin().lock())).for_each(|line| {
        let mut split = line.split_whitespace();
        let a = split.next().and_then(|s| s.parse::<i32>().ok());
        let b = split.next().and_then(|s| s.parse::<i32>().ok());
        match (a, b) {
            (Some(x), Some(y)) => {
                xs.push(x);
                ys.push(y);
                yc.entry(y).and_modify(|e| *e += 1).or_insert(1);
            },
            (_, _) => panic!("Bad string? {}", line),
        }
    });

    xs.sort();
    ys.sort();

    let p1:i32 = xs.iter().zip(ys.iter()).map(|(a, b)| { (a - b).abs() }).sum();
    let p2:i32 = xs.iter().map(|x| x * yc.get(x).unwrap_or(&0)).sum();
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
