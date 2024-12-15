use std::iter::repeat;
use util::read_input;

fn read_disk() -> Vec<i64> {
    read_input()
        .take(1)
        .next()
        .unwrap()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i64)
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Disk {
    File(i64),
    Empty,
}

fn compact(disk: &mut Vec<Disk>) {
    let mut i = 0;
    let mut j = disk.len() - 1;
    while i < j {
        match (disk[i], disk[j]) {
            (Disk::File(_), Disk::Empty) => {
                i = i + 1;
                j = j - 1;
            }
            (Disk::File(_), Disk::File(_)) => {
                i = i + 1;
            }
            (Disk::Empty, Disk::Empty) => {
                j = j - 1;
            }
            (Disk::Empty, Disk::File(_)) => {
                disk.swap(i, j);
                i = i + 1;
                j = j - 1;
            }
        }
    }
}

fn solve_p1(disk: &Vec<i64>) -> i64 {
    let mut fileno = 0..;
    let mut expanded: Vec<Disk> = disk
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            if 0 == i % 2 {
                repeat(Disk::File(fileno.next().unwrap())).take(*x as usize)
            } else {
                repeat(Disk::Empty).take(*x as usize)
            }
        })
        .collect();
    compact(&mut expanded);
    expanded
        .iter()
        .filter_map(|d| match d {
            Disk::File(n) => Some(n),
            Disk::Empty => None,
        })
        .zip(0..)
        .map(|(a, b)| (a * b) as i64)
        .sum()
}

#[derive(Debug)]
enum Disk2 {
    File(i64, i64),
    Empty(i64),
}

// this is all wrong just start over.
fn solve_p2(disk: &Vec<i64>) -> i64 {
    let orig: Vec<Disk2> = disk
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if 0 == i % 2 {
                Disk2::File(*x, 0)
            } else {
                Disk2::Empty(*x)
            }
        })
        .collect();
    let mut fileno = 0..;
    let mut expanded: Vec<Disk2> = orig
        .into_iter()
        .map(|x| match x {
            Disk2::File(count, _) => Disk2::File(count, fileno.next().unwrap()),
            Disk2::Empty(count) => Disk2::Empty(count),
        })
        .collect();

    let mut j = expanded.len() - 1;
    while j > 0 {
        if let Disk2::File(fsize, fno) = expanded[j] {
            for i in 0..j {
                if let Disk2::Empty(esize) = expanded[i] {
                    if fsize == esize {
                        expanded.swap(i, j);
                        break;
                    } else if fsize < esize {
                        expanded[i] = Disk2::File(fsize, fno);
                        expanded[j] = Disk2::Empty(fsize);
                        expanded.insert(1 + i, Disk2::Empty(esize - fsize));
                        j += 1;
                        break;
                    }
                }
            }
        }
        j -= 1;
    }
    expanded
        .iter()
        .filter_map(|x| match x {
            Disk2::File(n, f) => Some(repeat(*f).take(*n as usize)),
            Disk2::Empty(n) => Some(repeat(-1 as i64).take(*n as usize)),
        })
        .flat_map(|x| x)
        .zip(0..)
        .map(|(a, b)| (a * b) as i64)
        .filter(|n| *n > 0)
        .sum()
}

fn main() {
    let disk = read_disk();
    let part1 = solve_p1(&disk);
    let part2 = solve_p2(&disk);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
