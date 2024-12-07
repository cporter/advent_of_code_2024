use std::io::{self, BufReader};
use regex::Regex;
use util::read_lines;
use std::fmt;

enum Instruction {
    Do,
    Dont,
    Mul {a:i64, b: i64}
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Do => write!(f, "Do"),
            Instruction::Dont => write!(f, "Don't"),
            Instruction::Mul{a,b} => write!(f, "Mul({}, {})", a, b)
        }
    }
}

struct Total {
    tot: i64,
    active: bool,
}

fn main() {
    let re1 = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re2 = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    let lines:Vec<String> = read_lines(BufReader::new(io::stdin().lock())).collect();

    let part1 :i64= lines.iter().flat_map(|x| re1.captures_iter(&x))
        .map(|capture| {
            let a = capture[1].parse::<i64>().ok();
            let b = capture[2].parse::<i64>().ok();
            match (a, b) {
                (Some(x), Some(y)) => x * y,
                _ => panic!("BAD NUMBERS"),
            }
        }).sum();
    println!("part 1: {}", part1);

    let part2 = lines.iter().flat_map(|x| re2.captures_iter(&x)).map(|capture| {
        match &capture[0] {
            "do()" => Instruction::Do{},
            "don't()" => Instruction::Dont{},
            _ => {
                let a = capture[2].parse::<i64>().ok().unwrap();
                let b = capture[3].parse::<i64>().ok().unwrap();
                Instruction::Mul{a,b}
            }
        }
    })
    .fold(Total{tot: 0, active: true}, |tot, inst: Instruction| {
        match inst {
            Instruction::Do => Total{tot:tot.tot, active:true},
            Instruction::Dont => Total{tot:tot.tot, active:false},
            Instruction::Mul{a, b} => {
                if tot.active {
                    Total{tot:tot.tot + a * b, active:tot.active}
                } else {
                    tot
                }
            }
        }
    }).tot;
    println!("part 2: {}", part2);
}