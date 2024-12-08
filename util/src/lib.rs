use std::io::{BufRead, BufReader};

pub fn read_lines(reader: BufReader<impl BufRead>) -> impl Iterator<Item = String> {
    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
}
