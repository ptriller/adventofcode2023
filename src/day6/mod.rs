use std::fs::read_to_string;
use std::path::Path;

fn calc_product(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut lines = data.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();
    let mut result = 1;
    for (time, record) in
    first_line[10..].split(' ').filter(|c| !c.is_empty()).map(|c| c.parse::<u64>().unwrap())
        .zip(second_line[10..].split(' ').filter(|c| !c.is_empty()).map(|c| c.parse::<u64>().unwrap()))
    {
        let range = calc_range(time, record);
        result *= range;
    }
    result
}

fn calc_range(time: u64, record: u64) -> u64 {
    // Solution by math
    let root = f64::sqrt((time * time - 4 * record) as f64);
    let lower = (0.5f64 * (time as f64 - root) + 1f64).floor() as u64;
    let upper = (0.5f64 * (root + time as f64) - 1f64).ceil() as u64;
    upper - lower + 1
}

fn calc_real_time(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut lines = data.lines();
    let mut time_line = lines.next().unwrap()[10..].to_string();
    time_line.retain(|c| !c.is_whitespace());
    let time = time_line.parse().unwrap();
    let mut record_line = lines.next().unwrap()[10..].to_string();
    record_line.retain(|c| !c.is_whitespace());
    let record = record_line.parse().unwrap();
    calc_range(time, record)
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day6::{calc_product, calc_real_time};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day6/input.txt");
        println!("Day 6, Problem 1: : Product {}", calc_product(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day6/input.txt");
        println!("Day 6, Problem 2: Real time: {}", calc_real_time(test_data.as_path()));
    }
}