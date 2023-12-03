use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::Enumerate;
use std::path::Path;
use std::str::{Chars};


fn calc_gear_ratios(path: &Path) -> u32 {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let test_data = read_to_string(path).unwrap();
    let data: Vec<&str> = test_data.lines().collect();
    for (row, line) in data.iter().enumerate() {
        let mut it = line.chars().enumerate();
        while let Some((col, chr)) = it.next() {
            if chr.is_ascii_digit() {
                let mut num = chr.to_digit(10).unwrap();
                let length = get_mumber(&mut num, &mut it);
                GearChecker::check(&mut gears, &data, row, col, length, num);
            }
        }
    }
    let mut result = 0;
    for (_k, v) in &gears {
        if v.len() == 2 {
            result += v.iter().product::<u32>();
        }
    }
    result
}

struct GearChecker<'life> {
    gears: &'life mut HashMap<(usize, usize), Vec<u32>>,
    data: &'life Vec<&'life str>,
    row: usize,
    col: usize,
    length: usize,
    value: u32,
}


impl GearChecker<'_> {
    fn check(gears: &mut HashMap<(usize, usize), Vec<u32>>,
             data: &Vec<&str>,
             row: usize,
             col: usize,
             length: usize,
             value: u32) {
        GearChecker {
            gears,
            data,
            row,
            col,
            length,
            value,
        }.register_gears()
    }

    fn check_char(&mut self, chr: char, pos: (usize, usize)) {
        if chr == '*' {
            self.gears.entry(pos)
                .and_modify(|v| v.push(self.value))
                .or_insert(vec![self.value]);
        }
    }
    fn register_gears(&mut self) {
        let from = 0i32.max(self.col as i32 - 1) as usize;
        let to = (self.data[0].len() - 1).min(self.col + self.length);
        if self.row > 0 {
            for (col, chr) in (from..=to).zip(&mut self.data[self.row - 1][from..=to].chars()) {
                self.check_char(chr, (self.row - 1, col));
            }
        }
        self.check_char(self.data[self.row].chars().nth(from).unwrap(), (self.row, from));
        self.check_char(self.data[self.row].chars().nth(to).unwrap(), (self.row, to));
        if self.row + 1 < self.data.len() {
            for (col, chr) in (from..=to).zip(&mut self.data[self.row + 1][from..=to].chars()) {
                self.check_char(chr, (self.row + 1, col));
            }
        }
    }
}


fn calc_serial_number(path: &Path) -> u32 {
    let mut result = 0;
    let test_data = read_to_string(path).unwrap();
    let data: Vec<&str> = test_data.lines().collect();
    for (row, line) in data.iter().enumerate() {
        let mut it = line.chars().enumerate();
        while let Some((col, chr)) = it.next() {
            if chr.is_ascii_digit() {
                let mut num = chr.to_digit(10).unwrap();
                let length = get_mumber(&mut num, &mut it);
                if is_serial(&data, row, col, length) {
                    result += num;
                }
            }
        }
    }
    result
}

fn is_symbol(chr: char) -> bool {
    chr != '.' && !chr.is_ascii_digit()
}

fn is_serial(data: &Vec<&str>, row: usize, col: usize, length: usize) -> bool {
    let from = 0i32.max(col as i32 - 1) as usize;
    let to = (data[0].len() - 1).min(col + length);
    if row > 0 && data[row - 1][from..=to].find(is_symbol).is_some() {
        return true;
    }
    if is_symbol(data[row].chars().nth(from).unwrap()) ||
        is_symbol(data[row].chars().nth(to).unwrap()) {
        return true;
    }
    if row + 1 < data.len() && data[row + 1][from..=to].find(is_symbol).is_some() {
        return true;
    }
    false
}

fn get_mumber(num: &mut u32, it: &mut Enumerate<Chars>) -> usize {
    let mut length = 1;
    while let Some((_, chr)) = it.next() {
        if !chr.is_ascii_digit() { break; }
        length += 1;
        *num = *num * 10 + chr.to_digit(10).unwrap();
    }
    length
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day3::{calc_gear_ratios, calc_serial_number};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day3/input.txt");
        println!("Day 3, Problem 1: Valid games: {}", calc_serial_number(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day3/input.txt");
        println!("Day 3, Problem 1: Valid games: {}", calc_gear_ratios(test_data.as_path()));
    }
}