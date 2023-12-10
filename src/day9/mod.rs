use std::fs::read_to_string;
use std::path::Path;

fn extrapolate_data(path: &Path) -> i32 {
    let mut result = 0;
    let data = read_to_string(path).unwrap();
    for line in data.lines() {
        let data: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let data = different_extend(&data);
        result += data;
    }
    result
}

fn pre_extrapolate_data(path: &Path) -> i32 {
    let mut result = 0;
    let data = read_to_string(path).unwrap();
    for line in data.lines() {
        let data: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let data = different_prepend(&data);
        result += data;
    }
    result
}

fn different_extend(data: &Vec<i32>) -> i32 {
    if data.iter().all(|x| *x == 0) {
        return 0;
    }
    let diff = (1..data.len()).map(|i| data[i] - data[i - 1]).collect();
    let val = different_extend(&diff);
    data.last().unwrap() + val
}

fn different_prepend(data: &Vec<i32>) -> i32 {
    if data.iter().all(|x| *x == 0) {
        return 0;
    }
    let diff = (1..data.len()).map(|i| data[i] - data[i - 1]).collect();
    let val = crate::day9::different_prepend(&diff);
    data.first().unwrap() - val
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day9::{extrapolate_data, pre_extrapolate_data};


    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day9/input.txt");
        println!("Day 9, Problem 1: Extrapolation {}", extrapolate_data(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day9/input.txt");
        println!("Day 9, Problem 2: Pre: {}", pre_extrapolate_data(test_data.as_path()));
    }
}