use std::fs::read_to_string;
use std::path::Path;

fn trace_area(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut ngrid = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut frontier = vec![find_start(&grid)];
    while !frontier.is_empty() {
        for (row, col) in &frontier {
            ngrid[*row][*col] = grid[*row][*col];
        }
        let new_frontier = scan_frontier(&mut grid, &frontier);
        frontier = new_frontier;
    }
    for line in &ngrid {
        println!("{:?}", line);
    }
    // FLood fill all outsides
    // count pipes in ngrid from left to right, if you find an "outside" that has not been filled
    // Flood fill ...
    // The rest is inside

    let mut area = 0;
    for row in 0..grid.len() {
        let mut inside = false;
        for col in 0..grid[row].len() {
            if ['S', '|', 'J', 'L'].contains(&ngrid[row][col]) {
                inside = !inside;
                continue;
            }
            if ngrid[row][col] == '.' && inside {
                grid[row][col] = 'I';
                area += 1;
            }
            if ngrid[row][col] == '.' && !inside {
                grid[row][col] = 'O';
            }
        }
    }
    for line in &grid {
        println!("{:?}", line);
    }
    area
}

fn trace_path(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut frontier = vec![find_start(&grid)];
    let mut distance = 0;
    while !frontier.is_empty() {
        let new_frontier = scan_frontier(&mut grid, &frontier);
        distance += 1;
        frontier = new_frontier;
    }
    for line in grid {
        println!("{:?}", line);
    }
    distance - 1
}

fn scan_frontier(grid: &mut Vec<Vec<char>>, frontier: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_frontier = vec![];
    for (row_r, col_r) in frontier {
        let (row, col) = (*row_r, *col_r);
        // TOP
        if row > 0 &&
            ['S', '|', 'L', 'J'].contains(&grid[row][col]) &&
            ['S', '|', '7', 'F'].contains(&grid[row - 1][col]) {
            new_frontier.push((row - 1, col));
        }
        // RIGHT
        if col + 1 < grid[row].len() &&
            ['S', '-', 'L', 'F'].contains(&grid[row][col]) &&
            ['S', '-', 'J', '7'].contains(&grid[row][col + 1]) {
            new_frontier.push((row, col + 1));
        }
        // BOTTOM
        if row + 1 < grid.len() &&
            ['S', '|', '7', 'F'].contains(&grid[row][col]) &&
            ['S', '|', 'L', 'J'].contains(&grid[row + 1][col]) {
            new_frontier.push((row + 1, col));
        }
        // LEFT
        if col > 0 &&
            ['S', '-', 'J', '7'].contains(&grid[row][col]) &&
            ['S', '-', 'L', 'F'].contains(&grid[row][col - 1]) {
            new_frontier.push((row, col - 1));
        }
        grid[row][col] = '#';
    }
    new_frontier
}

fn find_start(data: &[Vec<char>]) -> (usize, usize) {
    for (row, line) in data.iter().enumerate() {
        for (col, chr) in line.iter().enumerate() {
            if *chr == 'S' { return (row, col); }
        }
    }
    panic!("NO start found")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day10::{trace_area, trace_path};


    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day10/input.txt");
        println!("Day 10, Problem 1: Distance {}", trace_path(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day10/input.txt");
        println!("Day 10, Problem 2: Area: {}", trace_area(test_data.as_path()));
    }
}