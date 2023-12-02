use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn calc_power_lines(filename: &Path) -> usize {
    let mut power = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let data = parse_line(line);
        let mut cnt: HashMap<&String, usize> = HashMap::new();
        for map in &data {
            for (k, v) in map.iter() {
                cnt.entry(k).and_modify(|i| *i = *v.max(i)).or_insert(*v);
            }
        }
        power += cnt.values().product::<usize>();
    }
    power
}

fn count_valid_lines(filename: &Path) -> usize {
    let mut valid = 0;
    let valid_data = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    'lineloop:
    for (line, linenum ) in read_to_string(filename).unwrap().lines().zip(1..) {
        let data = parse_line(line);
        for map in data {
            for (k, v) in map.iter() {
                if v > valid_data.get(k.as_str()).unwrap() { continue 'lineloop; }
            }
        }
        valid += linenum;
    }
    valid
}

fn parse_line(line: &str) -> Vec<HashMap<String, usize>> {
    let mut result_vec = vec![];
    let start = line.find(':').unwrap() + 2;
    let data = &line[start..];
    for seg in data.split(';') {
        let mut data = HashMap::new();
        for ball in seg.split(',') {
            let split: Vec<&str> = ball.trim().split(' ').collect();
            data.insert(split[1].to_string(),
                        split[0].parse().unwrap());
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