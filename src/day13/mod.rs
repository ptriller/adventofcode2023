use std::fs::read_to_string;
use std::path::Path;

fn calc_mirror_data(path: &Path, error: usize) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut zones = vec![];
    let mut zone: Vec<&str> = vec![];
    for line in data.lines() {
        if line.is_empty() {
            zones.push(zone);
            zone = vec![];
        } else {
            zone.push(line);
        }
    }
    zones.push(zone);
    zones.iter().map(|a
    | calc_zone_mirror(a, error)).sum()
}

fn calc_zone_mirror(zone: &[&str], error: usize) -> u32 {
    let mut result = 0;
    for row in 0..zone.len() - 1 {
        if check_mirror_row(row, zone, error) {
            result += 100 * (1 + row);
        }
    }
    let mut col = vec![String::new(); zone[0].len()];
    for row in zone {
        row.chars().enumerate().for_each(|(i, c)| col[i].push(c))
    }
    let izone: Vec<&str> = col.iter().map(String::as_str).collect();
    for col in 0..zone[0].len() - 1 {
        if check_mirror_row(col, &izone, error) {
            result += 1 + col;
        }
    }
    result as u32
}

fn check_mirror_row(row: usize, zone: &[&str], error: usize) -> bool {
    let mut top = row;
    let mut bottom = row + 1;
    let mut fault = 0;
    loop {
        fault += zone[top].chars().zip(zone[bottom].chars()).filter(|(l, r)| l != r).count();
        if fault > error { return false; }
        if top == 0 || bottom + 1 == zone.len() { return fault == error; }
        bottom += 1;
        top -= 1;
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day13::calc_mirror_data;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day13/input.txt");
        println!("Day 13, Problem 1: Mirror {}", calc_mirror_data(test_data.as_path(), 0));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day13/input.txt");
        println!("Day 13, Problem 2: Mirror: {}", calc_mirror_data(test_data.as_path(), 1));
    }
}