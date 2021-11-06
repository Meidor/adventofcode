#![feature(test)]
extern crate test;

use std::path::Path;

use solutions::day1;
mod solutions;

fn main() {
    let path = Path::new("./inputs/day1.txt");
    println!("Part one:");
    day1::part_one(solutions::read_lines(path).unwrap()).unwrap();

    println!("Part two:");
    day1::part_two(solutions::read_ints(path).unwrap()).unwrap();
}
