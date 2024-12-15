use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(reader: BufReader<impl BufRead>) -> impl Iterator<Item = String> {
    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
}

pub fn read_input() -> Box<dyn Iterator<Item = String>> {
    if let Some(filename) = env::args().nth(1) {
        match File::open(&filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                Box::new(reader.lines().filter_map(Result::ok))
            }
            Err(e) => {
                panic!("Error opening file: {}", e);
            }
        }
    } else {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());
        Box::new(reader.lines().filter_map(Result::ok))
    }
}

pub type Grid = Vec<Vec<char>>;

pub fn read_chargrid() -> Grid {
    read_input().map(|row| row.chars().collect()).collect()
}

pub type Coord = (usize, usize);

pub fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<Coord> {
    let mut ret = Vec::new();
    if row > 0 {
        ret.push((row - 1, col));
    }
    if col > 0 {
        ret.push((row, col - 1))
    }
    if row < (rows - 1) {
        ret.push((row + 1, col))
    }
    if col < (cols - 1) {
        ret.push((row, col + 1))
    }
    ret
}
