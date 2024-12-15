use std::collections::{HashSet, VecDeque};
use util::{neighbors, read_chargrid, Coord, Grid};

type Plot = (char, HashSet<Coord>);

fn flood(g: &Grid, ch: char, start: Coord, seen: &mut HashSet<Coord>) -> HashSet<Coord> {
    let rows = g.len();
    let cols = g[0].len();
    let mut ret: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<Coord> = VecDeque::new();
    q.push_back(start);
    while !q.is_empty() {
        if let Some((row, col)) = q.pop_back() {
            let coord = (row, col);
            if !seen.contains(&coord) {
                seen.insert(coord);
                ret.insert(coord);
                for (r, c) in neighbors(row, col, rows, cols) {
                    let ncoord = (r, c);
                    if g[r][c] == ch && !seen.contains(&ncoord) {
                        q.push_back(ncoord);
                    }
                }
            }
        }
    }
    ret
}

fn find_plots(g: &Grid, seen: &mut HashSet<Coord>) -> Vec<Plot> {
    let mut ret: Vec<Plot> = Vec::new();
    for (ri, row) in g.iter().enumerate() {
        for (ci, ch) in row.iter().enumerate() {
            let coord = (ri, ci);
            if !seen.contains(&coord) {
                let plot = flood(g, *ch, coord, seen);
                if 0 < plot.len() {
                    ret.push((*ch, plot));
                }
            }
        }
    }
    ret
}

fn size(p: &Plot, g: &Grid) -> (usize, usize) {
    let (ch, coords) = p;
    let mut perimeter: usize = 0;
    let rows = g.len();
    let cols = g[0].len();
    for (row, col) in coords.iter() {
        if *row == 0 || *row == (rows - 1) {
            perimeter += 1;
        }
        if *col == 0 || *col == (cols - 1) {
            perimeter += 1;
        }
        for (r, c) in neighbors(*row, *col, rows, cols) {
            if g[r][c] != *ch {
                perimeter += 1;
            }
        }
    }
    (coords.len(), perimeter)
}

fn main() {
    let mut seen: HashSet<Coord> = HashSet::new();
    let g = read_chargrid();
    let plots = find_plots(&g, &mut seen);
    let part1: usize = plots
        .iter()
        .map(|p| {
            let (area, perim) = size(&p, &g);
            area * perim
        })
        .sum();
    println!("part 1: {}", part1);
}
