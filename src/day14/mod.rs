use std::fs::read_to_string;
use std::path::Path;

fn calc_north_load(path: &Path) -> u32 {
    let mut data = load_board(path);
    tilt_north(&mut data);
    for datum in &data {
        println!("{:?}", datum.iter().map(|c| *c as char).collect::<Vec<char>>());
    }
    calc_load(&data)
}

fn calc_load(board: &[Vec<u8>]) -> u32 {
    board.iter().enumerate()
        .map(|(i, l)|
                 (l.iter().filter(|c| **c == b'O').count() * (board.len() - i)) as u32
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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day14::calc_north_load;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day14/input.txt");
        println!("Day 14, Problem 1: Mirror {}", calc_north_load(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day14/testinput.txt");
        println!("Day 14, Problem 2: Mirror: {}", 0);
    }
}