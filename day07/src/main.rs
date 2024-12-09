use util::read_input;

struct Problem {
    target: i64,
    elements: Vec<i64>,
}

fn parse_line(line: String) -> Option<Problem> {
    let parts: Vec<&str> = line.split(": ").collect();
    if 2 != parts.len() {
        return None;
    }
    let target = parts[0].trim().parse::<i64>().ok()?;
    let mut elements: Vec<i64> = parts[1]
        .split_whitespace()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    elements.reverse();
    Some(Problem {
        target: target,
        elements: elements,
    })
}

fn solvable_s_1(s: &[i64], target: i64) -> bool {
    match s {
        [] => false,
        [x] => *x == target,
        [first, rest @ ..] => {
            (target % first == 0 && solvable_s_1(rest, target / first))
                || solvable_s_1(rest, target - first)
        }
    }
}

fn solvable_1(p: &Problem) -> bool {
    solvable_s_1(&p.elements.as_slice(), p.target)
}

fn ends_with_same_digits(aa: i64, bb: i64) -> bool {
    let mut a = aa;
    let mut b = bb;
    while a > 0 && b > 0 {
        if a % 10 != b % 10 {
            return false;
        }
        a /= 10;
        b /= 10;
    }
    return true;
}

fn truncate_digits(aa: i64, bb: i64) -> i64 {
    let mut a = aa;
    let mut b = bb;
    while a > 0 && b > 0 {
        a /= 10;
        b /= 10;
    }
    return a;
}

fn solvable_s_2(s: &[i64], target: i64) -> bool {
    match s {
        [] => false,
        [x] => *x == target,
        [first, rest @ ..] => {
            (target % first == 0 && solvable_s_2(rest, target / first))
                || (ends_with_same_digits(target, *first)
                    && solvable_s_2(rest, truncate_digits(target, *first)))
                || solvable_s_2(rest, target - first)
        }
    }
}

fn solvable_2(p: &Problem) -> bool {
    solvable_s_2(&p.elements.as_slice(), p.target)
}

fn main() {
    let problems: Vec<Problem> = read_input().map(parse_line).filter_map(|x| x).collect();
    let part1 = problems
        .iter()
        .filter(|p| solvable_1(p))
        // .map(|p: &Problem| {
        //     println!("{} = {:?}", p.target, p.elements);
        //     p
        // })
        .map(|p| p.target)
        .sum::<i64>();
    println!("part 1: {}", part1);

    let part2 = problems
        .iter()
        .filter(|p| solvable_2(p))
        // .map(|p: &Problem| {
        //     println!("{} = {:?}", p.target, p.elements);
        //     p
        // })
        .map(|p| p.target)
        .sum::<i64>();
    println!("part 2: {}", part2);
}
