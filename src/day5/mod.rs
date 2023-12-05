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
    source: Range<u64>,
    target: Range<u64>,
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
    let mut vtype = "seed";
    let mut ranges: Vec<Range<u64>> = almanac.seeds
        .chunks(2)
        .map(|c| c[0]..(c[0] + c[1])).collect();
    while vtype != "location" {
        let mut new_ranges = vec![];
        let maps = almanac.mappings.get(vtype).unwrap();
        'next_range:
        while let Some(brange) = ranges.pop() {
            let mut range = brange;
            for mapping in &maps.maps {
                let (b, c, a) =
                    intersect_ranges(&range, &mapping.source);
                if let Some(r) = b {
                    assert!(r.end >= r.start);
                    new_ranges.push(r);
                }
                if let Some(r) = c {
                    assert!(r.end >= r.start);
                    new_ranges.push(r.start + mapping.target.start - mapping.source.start..
                        r.end + mapping.target.start - mapping.source.start
                    );
                }
                if let Some(r) = a {
                    assert!(r.end >= r.start);
                    range = r;
                } else {
                    continue 'next_range;
                }
            }
            new_ranges.push(range);
        }
        ranges = new_ranges;
        vtype = &maps.ttype;
    }
    ranges.iter().map(|r| r.start).min().unwrap()
}

impl Section {
    fn map(&self, input: u64) -> u64 {
        for ref map in &self.maps {
            if input >= map.source.start && input < map.source.end {
                let result = input + map.target.start - map.source.start;
                return result;
            }
        }
        input
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
        let numbers = number_list(line);
        assert_eq!(3, numbers.len());
        maps.push(Mapping {
            source: numbers[1]..(numbers[1] + numbers[2]),
            target: numbers[0]..(numbers[0] + numbers[2]),
        });
    }
    maps.sort_unstable_by_key(|m: &Mapping| m.source.start);
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


fn intersect_ranges(input: &Range<u64>, base: &Range<u64>) -> (Option<Range<u64>>,
                                                               Option<Range<u64>>,
                                                               Option<Range<u64>>) {
    // Before
    if input.end <= base.start {
        return (Some(input.clone()), None, None);
    }
    // Before and in
    if input.start <= base.start
        && input.end > base.start
        && input.end <= base.end {
        return (Some(input.start..base.start).filter(|x| x.end > x.start),
                Some(base.start..input.end).filter(|x| x.end > x.start),
                None
        );
    }
    // Before and over
    if input.start <= base.start && input.end > base.end {
        return (Some(input.start..base.start).filter(|x| x.end > x.start),
                Some(base.start..input.end).filter(|x| x.end > x.start),
                Some(base.end..input.end).filter(|x| x.end > x.start)
        );
    }
    // In and in
    if input.start >= base.start
        && input.start < base.end
        && input.end <= base.end {
        return (
            None,
            Some(input.clone()),
            None
        );
    }
    // In and over
    if input.start >= base.start
        && input.start < base.end
        && input.end > base.end {
        return (
            None,
            Some(input.start..base.end).filter(|x| x.end > x.start),
            Some(base.end..input.end).filter(|x| x.end > x.start)
        );
    }
    // After
    if input.start >= base.end {
        return (None, None, Some(input.clone()));
    }
    panic!("Seems I missed a case")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::day5::{find_first_location, find_real_first_location};

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day5/input.txt");
        println!("Day 5, Problem 1: First Location: {}", find_first_location(test_data.as_path()));
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day5/input.txt");
        println!("Day 5, Problem 1: First Location: {}", find_real_first_location(test_data.as_path()));
    }
}