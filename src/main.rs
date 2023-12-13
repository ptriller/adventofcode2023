#![allow(dead_code)]

use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {

    println!("Hello, world!");
    let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_data.push("resources/day12/input.txt");
    println!("Day 11, Problem 1: Permutations {}", crate::day12::calc_unfolded_permutations(test_data.as_path()));

}
