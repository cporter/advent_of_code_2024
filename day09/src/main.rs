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

fn main() {
    let disk = read_disk();
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
    let part1: i64 = expanded
        .iter()
        .filter_map(|d| match d {
            Disk::File(n) => Some(n),
            Disk::Empty => None,
        })
        .zip(0..)
        .map(|(a, b)| (a * b) as i64)
        .sum();
    println!("part 1: {}", part1);
}
