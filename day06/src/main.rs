use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use util::read_input;

enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Orientation {
    fn from_char(ch: char) -> Option<Orientation> {
        match ch {
            '^' => Some(Orientation::UP),
            'v' => Some(Orientation::DOWN),
            '<' => Some(Orientation::LEFT),
            '>' => Some(Orientation::RIGHT),
            _ => None,
        }
    }
}

fn turn(o: &Orientation) -> Orientation {
    match o {
        Orientation::DOWN => Orientation::LEFT,
        Orientation::LEFT => Orientation::UP,
        Orientation::UP => Orientation::RIGHT,
        Orientation::RIGHT => Orientation::DOWN,
    }
}

#[derive(Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Eq for Coord {}

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

fn mv(c: Coord, o: &Orientation) -> Coord {
    match o {
        Orientation::DOWN => Coord {
            row: c.row + 1,
            col: c.col,
        },
        Orientation::UP => Coord {
            row: c.row - 1,
            col: c.col,
        },
        Orientation::LEFT => Coord {
            row: c.row,
            col: c.col - 1,
        },
        Orientation::RIGHT => Coord {
            row: c.row,
            col: c.col + 1,
        },
    }
}

struct Board {
    board: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(board: Vec<Vec<char>>) -> Self {
        let r = board.len();
        let c = board[0].len();
        Board {
            board: board,
            rows: r,
            cols: c,
        }
    }
    fn start(&self) -> Option<(Coord, Orientation)> {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if matches!(self.board[r][c], '<' | '>' | '^' | 'v') {
                    let o = match Orientation::from_char(self.board[r][c]) {
                        Some(val) => val,
                        None => panic!(
                            "Logic error. Could not read orientation {}",
                            self.board[r][c]
                        ),
                    };
                    return Some((
                        Coord {
                            row: r as i32,
                            col: c as i32,
                        },
                        o,
                    ));
                }
            }
        }
        None
    }
    fn next_blocked(&self, oc: Coord, o: &Orientation) -> bool {
        let c = mv(oc, o);
        self.board[c.row as usize][c.col as usize] == '#'
    }
    fn next_out(&self, oc: Coord, o: &Orientation) -> bool {
        let c = mv(oc, o);
        c.row < 0 || c.col < 0 || c.row >= self.rows as i32 || c.col >= self.cols as i32
    }
    fn show(&self, seen: &HashSet<Coord>) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let coord = Coord {
                    row: r as i32,
                    col: c as i32,
                };
                if seen.contains(&coord) {
                    print!("X");
                } else {
                    print!("{}", self.board[r][c]);
                }
            }
            println!();
        }
    }
}

fn main() {
    let board = Board::new(read_input().map(|line| line.chars().collect()).collect());
    if let Some((mut guard, mut orientation)) = board.start() {
        let mut seen: HashSet<Coord> = HashSet::new();
        loop {
            seen.insert(guard);
            while board.next_blocked(guard, &orientation) {
                orientation = turn(&orientation);
            }
            guard = mv(guard, &orientation);
            if board.next_out(guard, &orientation) {
                break;
            }
        }
        let part1 = seen.len();
        println!("part 1: {}", part1);
        // board.show(&seen);
    }
}
