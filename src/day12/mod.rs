use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) fn calc_permutations(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut springs = vec![];
    for line in data.lines() {
        let mut split = line.split(' ');
        let left: Vec<char> = split.next().unwrap().chars().collect();
        let right: Vec<u64> = split.next().unwrap().split(',')
            .map(|c| c.parse().unwrap()).collect();
        springs.push((left, right));
    }
    let mut result = 0;
    for (left, right) in springs {
        let perm = check_permutations(&left, &right, &mut HashMap::new());
        result += perm;
    }
    result
}


pub(crate) fn calc_unfolded_permutations(path: &Path) -> u64 {
    let data = read_to_string(path).unwrap();
    let mut springs = vec![];
    for line in data.lines() {
        let mut split = line.split(' ');
        let left: Vec<char> = split.next().unwrap().chars().collect();
        let right: Vec<u64> = split.next().unwrap().split(',')
            .map(|c| c.parse().unwrap()).collect();
        let (mut rleft, mut rright) = (vec![], vec![]);
        for i in 0..5 {
            if i > 0 {
                rleft.push('?');
            }
            rleft.extend(&left);
            rright.extend(&right);
        }
        springs.push((rleft, rright));
    }
    let mut result = 0;
    for (left, right) in springs.iter() {
        let perm = check_permutations(left, right, &mut HashMap::new());
        result += perm;
    }
    result
}

fn check_permutations(left: &[char], right: &[u64], cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(perm) = cache.get(&(left.len(), right.len())) {
        return *perm;
    }
    if right.is_empty() {
        return if left.iter().all(|c| *c != '#') { 1 } else { 0 };
    }
    let next = *right.first().unwrap() as usize;
    if next > left.len() { return 0; }
    let mut permutations = 0;
    for i in 0..=(left.len() - next) {
        let slice = &left[i..];
        let scount = right.iter().sum::<u64>() as usize;
        if slice.len() < scount + right.len() - 1 ||
            slice.iter().filter(|c| **c != '.').count() < scount ||
            slice.iter().filter(|c| **c == '#').count() > scount
        { break; }
        if slice[0] == '.' { continue; }
        if (slice[..next]).iter().all(|c| *c != '.') {
            // Done and at the end of the string
            if slice.len() == next {
                if right.len() == 1 { permutations += 1; }
                break;
            }
            if slice[next] != '#' {
                permutations += check_permutations(&slice[(1 + next)..], &right[1..], cache);
            }
        }
        if slice[0] == '#' { break; }
    }
    cache.insert((left.len(), right.len()), permutations);
    permutations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day12::{calc_permutations, calc_unfolded_permutations};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day12/input.txt");
        println!("Day 11, Problem 1: Permutations {}", calc_permutations(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day12/input.txt");
        println!("Day 11, Problem 2: Permutations: {}", calc_unfolded_permutations(test_data.as_path()));
    }
}