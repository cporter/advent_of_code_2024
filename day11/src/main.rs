use std::collections::HashMap;
use util::read_input;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Stones {
    One(i64),
    Two(i64, i64),
}

type Cache = HashMap<(Stones, usize), i64>;

fn ndigits(x: i64) -> i32 {
    (x as f64).log(10.0).floor() as i32 + 1
}

fn rules(x: i64) -> Stones {
    let nd = ndigits(x);
    if 0 == x {
        Stones::One(1)
    } else if 0 == nd % 2 {
        let tens = 10.0f64.powi(nd / 2) as i64;
        Stones::Two(x / tens, x % tens)
    } else {
        Stones::One(2024 * x)
    }
}

fn blink(x: Stones, depth: usize, cache: &mut Cache) -> i64 {
    if let Some(ret) = cache.get(&(x, depth)) {
        *ret
    } else if 0 == depth {
        let ret = match x {
            Stones::Two(_, _) => 2,
            Stones::One(_) => 1,
        };
        cache.insert((x, depth), ret);
        ret
    } else {
        let ret = match x {
            Stones::One(a) => blink(rules(a), depth - 1, cache),
            Stones::Two(a, b) => {
                blink(rules(a), depth - 1, cache) + blink(rules(b), depth - 1, cache)
            }
        };
        cache.insert((x, depth), ret);
        ret
    }
}

fn main() {
    let input = read_input()
        .map(|line| {
            line.split(" ")
                .map(|s| Stones::One(s.parse::<i64>().unwrap()))
                .collect::<Vec<Stones>>()
        })
        .next()
        .unwrap();

    let mut cache: Cache = Cache::new();
    let part1: i64 = input
        .iter()
        .map(|stone| blink(*stone, 25, &mut cache))
        .sum();

    let part2: i64 = input
        .iter()
        .map(|stone| blink(*stone, 75, &mut cache))
        .sum();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
