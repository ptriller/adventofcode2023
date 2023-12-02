use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn calc_power_lines(filename: &Path) -> usize {
    let mut power = 0;
    let mut linenum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        linenum += 1;
        let data = parse_line(linenum, line);
        let mut cnt: HashMap<&String, usize> = HashMap::new();
        for map in &data {
            for (k, v) in map.iter() {
                cnt.entry(k).and_modify(|i| *i = *v.max(i)).or_insert(*v);
            }
        }
        power += cnt.values().fold(1, |a, b| a * b);
    }
    power
}

fn count_valid_lines(filename: &Path) -> usize {
    let mut linenum = 0;
    let mut valid = 0;
    'lineloop:
    for line in read_to_string(filename).unwrap().lines() {
        linenum += 1;
        let data = parse_line(linenum, line);
        for map in data {
            for (k, v) in map.iter() {
                match k.as_str() {
                    "red" => if v > &12 { continue 'lineloop; }
                    "blue" => if v > &14 { continue 'lineloop; }
                    "green" => if v > &13 { continue 'lineloop; }
                    _ => { assert!(false); }
                }
            }
        }
        valid += linenum;
    }
    valid
}

fn parse_line(idx: usize, line: &str) -> Vec<HashMap<String, usize>> {
    let mut result_vec = vec![];
    let pat = regex::Regex::new("^\\s*(\\d+)\\s+(\\w+)\\s*$").unwrap();
    let start = 7 + format!("{}", idx).len();
    let data = &line[start..];
    for seg in data.split(';') {
        let mut data = HashMap::new();
        for ball in seg.split(',') {
            let result = pat.captures(ball).unwrap();
            data.insert(result.get(2).unwrap().as_str().to_string(),
                        result.get(1).unwrap().as_str().parse().unwrap());
        }
        result_vec.push(data);
    }
    result_vec
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day2/data1.txt");
        println!("Valid games: {}", count_valid_lines(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day2/data1.txt");
        println!("Power of games: {}", calc_power_lines(test_data.as_path()));
    }
}