use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn calc_path_length(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut lines = data.lines();
    let instrunctions = lines.next().unwrap();
    lines.next();
    let mut graph = HashMap::new();
    for line in lines {
        let key = &line[0..3];
        let val = (&line[7..10], &line[12..15]);
        graph.insert(key, val);
    }
    let mut pos = "AAA";
    let mut steps = 0u64;
    loop {
        for dir in instrunctions.chars() {
            match dir {
                'L' => pos = graph.get(pos).unwrap().0,
                'R' => pos = graph.get(pos).unwrap().1,
                _ => panic!("Illegal Data")
            }
            steps += 1;
            if pos == "ZZZ" { return steps; }
        }
    }
}

struct Node<'l> {
    left: &'l str,
    right: &'l str,

}

// This works ... but I have no idea why each starting point has exactly one loop.
// Is this math or is this good selection of data ?
fn calc_multi_path_length(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut lines = data.lines();
    let instrunctions = lines.next().unwrap();
    let mut start_pos = vec![];
    lines.next();
    let mut graph = HashMap::new();
    for line in lines {
        let key = &line[0..3];
        if key.ends_with('A') {
            start_pos.push(key);
        }
        let val = (&line[7..10], &line[12..15]);
        graph.insert(key, val);
    }
    let mut loop_size = vec![];
    'outer:
    for pos in start_pos {
        let mut my_pos = pos;
        let mut steps = 0u64;
        loop {
            for dir in instrunctions.chars() {
                my_pos = match dir {
                    'L' => graph.get(my_pos).unwrap().0,
                    'R' => graph.get(my_pos).unwrap().1,
                    _ => panic!("Illegal Data")
                };
                steps += 1;
                if my_pos.ends_with('Z') {
                    println!("{} Steps: {}", my_pos, steps);
                    loop_size.push(steps);
                    continue 'outer;
                }
            }
        }
    }
    loop_size.iter().copied().reduce(lcm).unwrap()
}

fn lcm(first: u64, second: u64) -> u64 {
    first * (second / gcd(first, second))
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day8::{calc_multi_path_length, calc_path_length};


    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day8/input.txt");
        println!("Day 8, Problem 1: Steps {}", calc_path_length(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day8/input.txt");
        println!("Day 8, Problem 2: Steps: {}", calc_multi_path_length(test_data.as_path()));
    }
}