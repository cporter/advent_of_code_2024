use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Bfs;
use util::read_chargrid;

#[derive(Debug)]
struct NodeData {
    val: u8,
}

type G = DiGraph<NodeData, i32>;

fn neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
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

fn read_graph() -> G {
    let grid = read_chargrid();
    let mut g = G::new();

    let ni: Vec<Vec<NodeIndex>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|ch| {
                    g.add_node(NodeData {
                        val: ch.to_digit(10).unwrap() as u8,
                    })
                })
                .collect()
        })
        .collect();

    let rows = ni.len();
    let cols = ni[0].len();
    for (ri, row) in ni.iter().enumerate() {
        for (ci, idx_0) in row.iter().enumerate() {
            for (i, j) in neighbors(ri, ci, rows, cols) {
                let idx_1 = ni[i][j];
                if g[*idx_0].val + 1 == g[idx_1].val {
                    g.add_edge(*idx_0, idx_1, 0);
                }
            }
        }
    }

    g
}

fn part_1(g: &G) -> i32 {
    g.node_indices()
        .filter(|ni| g[*ni].val == 0)
        .map(|ni| {
            let mut bfs = Bfs::new(g, ni);
            let mut ret = 0;
            while let Some(node) = bfs.next(g) {
                if g[node].val == 9 {
                    ret += 1
                }
            }
            ret
        })
        .sum()
}

fn main() {
    let g: G = read_graph();
    let part1 = part_1(&g);
    println!("part 1: {}", part1);
}
