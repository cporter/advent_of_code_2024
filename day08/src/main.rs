use itertools::Itertools;
use std::collections::HashSet;
use util::read_input;

type Grid = Vec<Vec<char>>;
type Letter = (char, i32, i32);

fn find_letters(g: &Grid) -> Vec<Letter> {
    let mut letters: Vec<Letter> = g
        .iter()
        .enumerate()
        .flat_map(move |(ri, row)| {
            row.iter()
                .enumerate()
                .map(move |(ci, letter)| (*letter, ri as i32, ci as i32))
        })
        .filter(|(letter, _, _)| *letter != '.')
        .collect();
    letters.sort();
    letters
}

fn main() {
    let grid: Grid = read_input()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let letters = find_letters(&grid);
    let mut stations: HashSet<(i32, i32)> = HashSet::new();
    let mut stations2: HashSet<(i32, i32)> = HashSet::new();

    let grouped = letters.into_iter().chunk_by(|&(ch, _, _)| ch); // Group by the char value

    // Iterate over the grouped results
    for (_, group) in &grouped {
        let chunks: Vec<(char, i32, i32)> = group.into_iter().collect();
        let pairs: Vec<((i32, i32), (i32, i32))> = chunks
            .iter()
            .flat_map(|(_, x1, y1)| {
                chunks.iter().filter_map(move |(_, x2, y2)| {
                    if (x1, y1) != (x2, y2) {
                        Some(((*x1, *y1), (*x2, *y2)))
                    } else {
                        None
                    }
                })
            })
            .collect();
        for ((x1, y1), (x2, y2)) in pairs {
            let dx = x2 - x1;
            let dy = y2 - y1;
            stations.insert((x1 - dx, y1 - dy));
            stations.insert((x2 + dx, y2 + dy));

            for i in 0.. {
                let x = x1 + dx * i;
                let y = y1 + dy * i;
                if x < 0 || y < 0 || x >= n || y >= m {
                    break;
                }
                stations2.insert((x, y));
            }
            // and the same thing but negative because lazy
            for i in 0.. {
                let x = x1 - dx * i;
                let y = y1 - dy * i;
                if x < 0 || y < 0 || x >= n || y >= m {
                    break;
                }
                stations2.insert((x, y));
            }
        }
    }

    let part1 = stations
        .iter()
        .filter(|(x, y)| x >= &0 && y >= &0 && x < &n && y < &m)
        .count();
    println!("part 1: {}", part1);
    let part2 = stations2.iter().count();
    println!("part 2: {}", part2);
}
