use std::fs::read_to_string;
use std::path::Path;

const EVEC: Vec<(&str, u32)> = Vec::new();

fn build_map(path: &Path) -> u32 {
    let data = read_to_string(path).unwrap();
    let mut map: [Vec<(&str, u32)>; 256] = [EVEC; 256];
    data.split(',')
        .for_each(|x| process_entry(x, &mut map));
    println!("{:?}", map);
    map.iter().enumerate()
        .map(|(i, d)|
            (i as u32 + 1) * d.iter().enumerate()
                .map(|(j, (_, f))| (j as u32 + 1) * f).sum::<u32>()
        ).sum::<u32>()
}

fn process_entry<'a>(entry: &'a str, hashmap: &mut [Vec<(&'a str, u32)>; 256]) {
    let idx = entry.find(|x: char| ['-', '='].contains(&x)).unwrap();
    let label = &entry[0..idx];
    let op = entry.chars().nth(idx).unwrap();
    let boxidx = str_sum(label);
    let hbox = &mut hashmap[boxidx as usize];
    if op == '-' {
        if let Some(idx) = hbox.iter().position(|(a, _)| *a == label) {
            hbox.remove(idx);
        }
    } else {
        let focus: u32 = entry[idx + 1..].parse().unwrap();
        if let Some(idx) = hbox.iter().position(|(a, _)| *a == label) {
            hbox[idx] = (label, focus);
        } else {
            hbox.push((label, focus));
        }
    }
}


fn calc_checksum(path: &Path) -> u32 {
    read_to_string(path)
        .unwrap()
        .split(',')
        .map(str_sum)
        .sum()
}

fn str_sum(data: &str) -> u32 {
    let mut result = 0;
    for char in data.chars() {
        result += char as u32;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day15/input.txt");
        println!("Day 15, Problem 1: Checksum {}", calc_checksum(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day15/input.txt");
        println!("Day 15, Problem 2: Checksum: {}", build_map(test_data.as_path()));
    }
}