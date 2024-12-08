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
