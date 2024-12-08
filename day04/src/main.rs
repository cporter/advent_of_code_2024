use std::io::{self, BufReader};
use util::read_lines;

fn word(row:i32, col:i32, ri: i32, ci: i32, len: i32) -> Vec<(i32,i32)>{
    (0..len).map(|i| (row + i * ri, col + i * ci)).collect()
}

fn words(row:i32, col:i32, len:i32) -> Vec<Vec<(i32, i32)>> {
    (-1..=1).flat_map(move |r| (-1..=1).map(move |c| {
        word(row, col, r, c, len)
    })).collect()
}

fn all_on_board(rows:&i32, cols:&i32, word:&Vec<(i32, i32)>) -> bool {
    word.iter().all(|(r, c)| {
        r >= &0 && c >= &0 && r < rows && c < cols
    })
}

fn spells(coords: &Vec<(i32, i32)>, letters:&Vec<Vec<char>>, word: &str) -> bool {
    let found:String = coords.iter().map(|(r,c)| letters[*r as usize][*c as usize]).collect();
    found == word
}


fn xword(row: i32, col: i32) -> Vec<(i32, i32)> {
    vec![(row-1, col-1), (row-1, col+1),
                (row, col),
        (row+1, col-1), (row+1, col+1)]
}

const ACCEPTABLE_XMAS:  [&'static str; 4] =[
    "MMASS", "SSAMM", "MSAMS", "SMASM"
];
fn xmas(coords: &Vec<(i32, i32)>, letters:&Vec<Vec<char>>) -> bool {
    let found:String = coords.iter().map(|(r,c)| letters[*r as usize][*c as usize]).collect();
    ACCEPTABLE_XMAS.iter().any(|cand| *cand == found)
}

fn main() {
    let lines:Vec<Vec<char>> = read_lines(BufReader::new(io::stdin().lock()))
        .map(|line| line.chars().collect())
        .collect();
    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    let coords: Vec<(i32, i32)> = (0..rows).flat_map(move |r| (0..cols).map(move |c| (r, c))).collect();

    let part1 = coords.iter().flat_map(|(r, c)| words(*r, *c, 4))
        .filter(|word| all_on_board(&rows, &cols, word))
        .filter(|word| spells(word, &lines, "XMAS"))
        .count();
    println!("part 1: {}", part1);

    let part2 = coords.iter().map(|(r, c)| xword(*r, *c))
        .filter(|word| all_on_board(&rows, &cols, word))
        .filter(|word| xmas(word, &lines))
        .count();
    println!("part 2: {}", part2);
}