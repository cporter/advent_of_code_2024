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

fn solvable_s(s: &[i64], target: i64) -> bool {
    match s {
        [] => false,
        [x] => *x == target,
        [first, rest @ ..] => {
            (target % first == 0 && solvable_s(rest, target / first))
                || solvable_s(rest, target - first)
        }
    }
}

fn solvable(p: &Problem) -> bool {
    return solvable_s(&p.elements.as_slice(), p.target);
}

fn main() {
    let problems: Vec<Problem> = read_input().map(parse_line).filter_map(|x| x).collect();
    let part1 = problems
        .iter()
        .filter(|p| solvable(p))
        .map(|p| p.target)
        .sum::<i64>();
    println!("part 1: {}", part1);
}
