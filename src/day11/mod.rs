use std::fs::read_to_string;
use std::path::Path;

#[allow(clippy::needless_range_loop)]
fn calc_distances(path: &Path, expansion: u64) -> u64 {
    let data = read_to_string(path).unwrap();
    let space: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut rows = vec![expansion; space.len()];
    let mut cols = vec![expansion; space[0].len()];
    for i in 0..rows.len() {
        if space[i].iter().any(|c| *c != '.') {
            rows[i] = 1;
        }
    }
    for i in 0..cols.len() {
        for j in 0..rows.len() {
            if space[j][i] != '.' {
                cols[i] = 1;
                break;
            }
        }
    }
    let mut galaxies = vec![];
    for (i, row) in space.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            if *chr == '#' {
                galaxies.push((i, j));
            }
        }
    }
    let mut result = 0;
    while let Some(left) = galaxies.pop() {
        for right in &galaxies {
            result += rows[left.0.min(right.0)..left.0.max(right.0)].iter().sum::<u64>()
                + cols[left.1.min(right.1)..left.1.max(right.1)].iter().sum::<u64>();
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day11::calc_distances;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day11/input.txt");
        println!("Day 11, Problem 1: Distance {}", calc_distances(test_data.as_path(), 2));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day11/input.txt");
        println!("Day 11, Problem 2: Distance: {}", calc_distances(test_data.as_path(), 1000000));
    }
}