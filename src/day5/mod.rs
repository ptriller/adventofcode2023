use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::ops::Range;
use std::path::Path;
use std::str::Lines;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<String, Section>,
}

#[derive(Debug)]
struct Section {
    stype: String,
    ttype: String,
    maps: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source: u64,
    target: u64,
    range: u64,
}

fn find_first_location(path: &Path) -> u64 {
    let almanac = Almanac::parse(path);
    let mut result = u64::MAX;
    for seed in almanac.seeds {
        let mut vtype = "seed";
        let mut value = seed;
        while vtype != "location" {
            let maps = almanac.mappings.get(vtype).unwrap();
            value = maps.map(value);
            vtype = &maps.ttype;
        }
        result = result.min(value);
    }
    result
}


fn find_real_first_location(path: &Path) -> u64 {
    let almanac = Almanac::parse(path);
    let mut result = u64::MAX;
    let mut ranges: Vec<Range<u64>> = vec![];

    for seed in almanac.seeds {
        let mut vtype = "seed";
        let mut value = seed;
        while vtype != "location" {
            let maps = almanac.mappings.get(vtype).unwrap();
            value = maps.map(value);
            vtype = &maps.ttype;
        }
        result = result.min(value);
    }
    result
}

impl Section {
    fn map(&self, input: u64) -> u64 {
        for ref map in &self.maps {
            if input >= map.source && input < map.source + map.range {
                let result = input - map.source + map.target;
                return result;
            }
        }
        input
    }

    fn range_map(&self, input: Range<u64>) -> Vec<(u64, u64)> {
        let mut result = vec![];

        result
    }
}

impl Almanac {
    fn parse(path: &Path) -> Almanac {
        let data = read_to_string(path).unwrap();
        let mut reader = data.lines().into_iter().peekable();
        let seed_line = reader.next().unwrap();
        assert!(seed_line.starts_with("seeds:"));
        let seeds = number_list(&seed_line[6..]);
        assert!(reader.next().unwrap().is_empty());
        let mut mappings = HashMap::new();
        while reader.peek().is_some() {
            let section = parse_section(&mut reader);
            mappings.insert(section.stype.clone(), section);
        }
        Almanac {
            seeds,
            mappings,
        }
    }
}

fn parse_section(reader: &mut Peekable<Lines>) -> Section {
    let header = reader.next().unwrap();
    assert!(header.ends_with(" map:"));
    let mut names = header[..header.len() - 5].split("-to-");
    let stype = names.next().unwrap().to_string();
    let ttype = names.next().unwrap().to_string();
    let mut maps = vec![];
    while let Some(line) = reader.next() {
        if line.is_empty() {
            break;
        }
        maps.sort_unstable_by_key(|m: &Mapping| m.source);
        let numbers = number_list(line);
        assert_eq!(3, numbers.len());
        maps.push(Mapping {
            target: numbers[0],
            source: numbers[1],
            range: numbers[2],
        });
    }
    Section {
        stype,
        ttype,
        maps,
    }
}


fn number_list(str: &str) -> Vec<u64> {
    str.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day5::find_first_location;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day5/input.txt");
        println!("Day 5, Problem 1: First Location: {}", find_first_location(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day4/testinput.txt");
        println!("Day 5, Problem 1: First Location: {}", 0);
    }
}