use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;
use hashbrown::HashMap;
use crate::day17::Direction::{HORIZONTAL, VERTICAL};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) struct Entry {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) struct HeapEntry {
    weight: isize,
    row: usize,
    col: usize,
    direction: Direction,
}

pub(crate) struct Data {
    minmove: isize,
    maxmove: isize,
    height: usize,
    width: usize,
    city: Vec<Vec<isize>>,
    tracking: HashMap<Entry, isize>,
    head: BinaryHeap<HeapEntry>,
}

impl HeapEntry {
    fn to_entry(&self) -> Entry {
        Entry {
            row: self.row,
            col: self.col,
            direction: self.direction,
        }
    }
}

impl Data {
    pub(crate) fn init(path: &Path, minmove: isize, maxmove: isize) -> Data {
        let city: Vec<Vec<isize>> = read_to_string(path).unwrap()
            .lines().map(|l| l.as_bytes().iter().map(|c| (c - b'0') as isize).collect())
            .collect();
        let (height, width) = (city.len(), city[0].len());
        Data {
            minmove,
            maxmove,
            height,
            width,
            head: BinaryHeap::new(),
            city,
            tracking: HashMap::new(),
        }
    }

    fn find_path(&mut self) -> u32 {
        self.head.push(HeapEntry {
            row: 0,
            col: 0,
            weight: 0,
            direction: HORIZONTAL,
        });
        self.tracking.insert(Entry {
            row: 0,
            col: 0,
            direction: HORIZONTAL,
        }, 0);
        while let Some(entry) = self.head.pop() {
            if (entry.row, entry.col) == (self.height - 1, self.width - 1) {
                return (-entry.weight) as u32;
            }

            if self.tracking.get(&entry.to_entry()).is_some_and(|c| -entry.weight > *c) {
                continue;
            }
            let directions = if entry.direction == HORIZONTAL {
                [(1, 0), (-1, 0)]
            } else {
                [(0, 1), (0, -1)]
            };
            for (r, c) in directions {
                let mut cost = -entry.weight;
                for dist in 1..=self.maxmove {
                    let nrow = (entry.row as isize + r * dist) as usize;
                    let ncol = (entry.col as isize + c * dist) as usize;
                    if nrow >= self.height || ncol >= self.width {
                        continue;
                    }
                    cost += self.city[nrow][ncol];
                    if dist < self.minmove {
                        continue;
                    }
                    let nhentry = HeapEntry {
                        row: nrow,
                        col: ncol,
                        weight: -cost,
                        direction: if entry.direction == HORIZONTAL { VERTICAL } else { HORIZONTAL },
                    };
                    let nentry = nhentry.to_entry();
                    if let Some(val) = self.tracking.get(&nentry) {
                        if *val <= cost { continue; }
                    }
                    self.tracking.insert(nentry, cost);
                    self.head.push(nhentry);
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day17/input.txt");
        let mut data = Data::init(test_data.as_path(), 1, 3);
        println!("Day 17, Problem 1: Loss: {}", data.find_path());
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day17/input.txt");
        let mut data = Data::init(test_data.as_path(), 4, 10);
        println!("Day 17, Problem 2: Loss: {}", data.find_path());
    }
}