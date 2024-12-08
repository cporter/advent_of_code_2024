use std::collections::HashMap;

use util::read_input;

fn parse_ordering(s: &String) -> (i32, i32) {
    let mut stuff = s.splitn(2, "|");
    let a = stuff.next().unwrap().parse::<i32>().unwrap();
    let b = stuff.next().unwrap().parse::<i32>().unwrap();
    (a, b)
}

fn parse_input() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut order: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut seen_blank = false;
    for line in read_input() {
        if line.is_empty() {
            seen_blank = true;
        } else if seen_blank {
            updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
        } else {
            let (k, v) = parse_ordering(&line);
            order.entry(k).or_insert_with(Vec::new).push(v);
        }
    }
    (order, updates)
}

fn ordered(update: &Vec<i32>, order: &HashMap<i32, Vec<i32>>) -> bool {
    update.iter().enumerate().all(|(i, &x)| {
        ((i + 1)..update.len())
            .map(|j| update[j])
            .all(|y| match order.get(&x) {
                Some(val) => val.contains(&y),
                None => false,
            })
    })
}

fn main() {
    let (order, updates) = parse_input();
    let part1: i32 = updates
        .iter()
        .filter(|update| ordered(update, &order))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("part 1: {}", part1);

    let part2: i32 = updates
        .iter()
        .filter(|update| !ordered(update, &order))
        .map(|update| {
            let mut with_counts: Vec<(i32, i32)> = update
                .iter()
                .map(|x| {
                    let count = match order.get(x) {
                        Some(after) => update.iter().filter(|y| after.contains(y)).count(),
                        None => 0,
                    };
                    (count as i32, *x)
                })
                .collect();
            with_counts.sort_by(|(a, _), (b, _)| b.cmp(a));
            with_counts.iter().map(|(_, x)| *x).collect::<Vec<i32>>()
        })
        .map(|update| update[update.len() / 2])
        .sum();
    println!("part 2: {}", part2);
}
