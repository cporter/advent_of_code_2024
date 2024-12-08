use std::collections::HashMap;
use std::io::{self, BufReader};

use util::read_lines;

fn parse_ordering(s:&String) -> (i32, i32) {
    let mut stuff = s.splitn(2, "|");
    let a = stuff.next().unwrap().parse::<i32>().unwrap();
    let b = stuff.next().unwrap().parse::<i32>().unwrap();
    (a, b)
}


fn main() {
    let mut order:HashMap<i32, Vec<i32>> = HashMap::new();
    let mut update:Vec<Vec<i32>> = Vec::new();
    let mut seen_blank = false;
    read_lines(BufReader::new(io::stdin().lock())).for_each(|line| {
        if line.is_empty() {
            seen_blank = true;
        } else if seen_blank {
            update.push(line.split(",").map(|x|x.parse::<i32>().unwrap()).collect());
        } else {
            let (k, v) = parse_ordering(&line);
            order.entry(k).or_insert_with(Vec::new).push(v);
        }
    });
    let part1 :i32 = update.iter()
        .filter(|update| {
            update.iter().enumerate().all(|(i, &x)| {
                ((i+1)..update.len()).map(|j| update[j]).all(|y| {
                    order.entry(x).or_default().contains(&y)
                })
            })
        })
        .map(|update| update[update.len()/2])
        .sum();
    println!("part 1: {}", part1);
}
