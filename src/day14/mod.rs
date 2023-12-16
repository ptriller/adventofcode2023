use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::path::Path;

// Worst code I have written so far .. duplicate calculations because of lazyness...

fn calc_extended_load(path: &Path) -> u64 {
    let (from, to) = find_loop(path);
    let length = to + 1 - from;
    let remainer = 1000000000 - from;
    let modu = remainer % length;
    println!("Modulo {modu}");
    let mut data = load_board(path);
    for _ in 0.. from+modu {
        tilt_north(&mut data);
        tilt_west(&mut data);
        tilt_south(&mut data);
        tilt_east(&mut data);
    }
    calc_load(&data)
}

fn find_loop(path: &Path) -> (u64, u64) {
    let mut states: HashMap<Vec<Vec<u8>>, u64> = HashMap::new();
    let mut data = load_board(path);
    let mut cycles = 0;
    loop {
//    while !states.contains_key(&data) {
        if !states.contains_key(&data) {
            states.insert(data.iter().map(|x| x.clone()).collect(), cycles);
        }
        tilt_north(&mut data);
        tilt_west(&mut data);
        tilt_south(&mut data);
        tilt_east(&mut data);
        if let Some(c) = states.get(&data) {
            return (*c, cycles);
        }
        cycles += 1;
    }
}

fn calc_north_load(path: &Path) -> u64 {
    let mut data = load_board(path);
    tilt_north(&mut data);
    for datum in &data {
        println!("{:?}", datum.iter().map(|c| *c as char).collect::<Vec<char>>());
    }
    calc_load(&data)
}

fn calc_load(board: &[Vec<u8>]) -> u64 {
    board.iter().enumerate()
        .map(|(i, l)|
            (l.iter().filter(|c| **c == b'O').count() * (board.len() - i)) as u64
        ).sum()
}

fn load_board(path: &Path) -> Vec<Vec<u8>> {
    read_to_string(path).unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect())
        .collect()
}

fn tilt_north(board: &mut [Vec<u8>]) {
    for col in 0..board[0].len() {
        let mut drop = 0;
        for row in 0..board.len() {
            match board[row][col] {
                b'#' => drop = row + 1,
                b'O' => {
                    if drop < row {
                        board[drop][col] = b'O';
                        board[row][col] = b'.';
                    }
                    drop += 1
                }
                b'.' => {}
                _ => panic!("Illegal character")
            }
        }
    }
}

fn tilt_south(board: &mut [Vec<u8>]) {
    for col in 0..board[0].len() {
        let mut drop = board.len() - 1;
        for row in (0..board.len()).rev() {
            match board[row][col] {
                b'#' => if row > 0 { drop = row - 1; }
                b'O' => {
                    if drop > row {
                        board[drop][col] = b'O';
                        board[row][col] = b'.';
                    }
                    if drop > 0 { drop -= 1; }
                }
                b'.' => {}
                _ => panic!("Illegal character")
            }
        }
    }
}

fn tilt_west(board: &mut [Vec<u8>]) {
    for row in 0..board.len() {
        let mut drop = 0;
        for col in 0..board[0].len() {
            match board[row][col] {
                b'#' => drop = col + 1,
                b'O' => {
                    if drop < col {
                        board[row][drop] = b'O';
                        board[row][col] = b'.';
                    }
                    drop += 1
                }
                b'.' => {}
                _ => panic!("Illegal character")
            }
        }
    }
}

fn tilt_east(board: &mut [Vec<u8>]) {
    for row in 0..board.len() {
        let mut drop = board[0].len() - 1;
        for col in (0..board[0].len()).rev() {
            match board[row][col] {
                b'#' => if col > 0 { drop = col - 1 }
                b'O' => {
                    if drop > col {
                        board[row][drop] = b'O';
                        board[row][col] = b'.';
                    }
                    if drop > 0 { drop -= 1 }
                }
                b'.' => {}
                _ => panic!("Illegal character")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day14/input.txt");
        println!("Day 14, Problem 1: Mirror {}", calc_north_load(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day14/input.txt");
        println!("Day 14, Problem 2: Mirror: {}", calc_extended_load(test_data.as_path()));
    }
}