use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use util::read_input;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: i32,
    col: i32,
}

type OrientedCoord = (Coord, Orientation);

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

enum SimResult {
    Escape(HashSet<Coord>),
    Loop(HashSet<Coord>),
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
    fn next_blocked(&self, oc: Coord, o: &Orientation, extra_block: Option<Coord>) -> bool {
        if self.next_out(oc, o) {
            return false;
        }
        let c = mv(oc, o);
        if let Some(block) = extra_block {
            if c == block {
                return true;
            }
        }
        self.board[c.row as usize][c.col as usize] == '#'
    }
    fn next_out(&self, oc: Coord, o: &Orientation) -> bool {
        let c = mv(oc, o);
        c.row < 0 || c.col < 0 || c.row >= self.rows as i32 || c.col >= self.cols as i32
    }

    fn sim(&self, extra_block: Option<Coord>) -> Option<SimResult> {
        if let Some((mut guard, orig_orientation)) = self.start() {
            let mut orientation = orig_orientation;
            let mut seen: HashSet<OrientedCoord> = HashSet::new();
            loop {
                if seen.contains(&(guard, orientation)) {
                    return Some(SimResult::Loop(
                        seen.into_iter().map(|(c, _)| c).collect::<HashSet<Coord>>(),
                    ));
                }

                while self.next_blocked(guard, &orientation, extra_block) {
                    orientation = turn(&orientation);
                }
                seen.insert((guard, orientation));
                if self.next_out(guard, &orientation) {
                    return Some(SimResult::Escape(
                        seen.into_iter().map(|(c, _)| c).collect::<HashSet<Coord>>(),
                    ));
                }
                guard = mv(guard, &orientation);
            }
        } else {
            None
        }
    }
}
fn main() {
    let board = Board::new(read_input().map(|line| line.chars().collect()).collect());
    match board.sim(None) {
        Some(SimResult::Escape(seen)) => {
            let part1 = seen.len();
            println!("part 1: {}", part1);

            let blocks = seen
                .iter()
                .filter(|coord| match board.sim(Some(**coord)) {
                    Some(SimResult::Escape(_)) => false,
                    Some(SimResult::Loop(_)) => true,
                    None => false,
                })
                .collect::<HashSet<&Coord>>();

            let part2 = blocks.len();
            println!("part 2: {}", part2);
        }
        Some(SimResult::Loop(_seen)) => {
            println!("Got a loop out of the first go?");
        }
        None => {
            println!("Just a plain error.");
        }
    }
}
