use std::fs::read_to_string;
use std::path::Path;

fn extract_number(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    for c in line.chars() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c.to_digit(10).unwrap())
            }
            last = Some(c.to_digit(10).unwrap())
        }
    }
    first.unwrap() * 10 + last.unwrap()
}

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn extract_extended_number(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    for i in 0..line.len() {
        let curr = &line[i..];
        for j in 0..9 {
            if curr.starts_with(NUMBERS[j]) || curr.starts_with(DIGITS[j]) {
                let val = j as u32 + 1;
                if first.is_none() {
                    first = Some(val)
                }
                last = Some(val)
            }
        }
    }
    first.unwrap() * 10 + last.unwrap()
}

fn process_file(filename: &Path) -> u32 {
    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let num = extract_number(line);
        assert_ne!(0, num);
        result += num;
    }
    result
}


fn process_extended_file(filename: &Path) -> u32 {
    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let num = extract_extended_number(line);
        assert_ne!(0, num);
        result += num;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day1::{process_extended_file, process_file};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day1/input.txt");
        println!("Day 1, Problem 1: Result: {}", process_file(test_data.as_path()))
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day1/input.txt");
        println!("Day 1, Problem 2: Result: {}", process_extended_file(test_data.as_path()))
    }
}