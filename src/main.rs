#![feature(test)]
#![allow(dead_code)]
extern crate test;

use std::path::Path;

use solutions::day1;
mod solutions;

fn day_1(){
    println!("DAY 1");
    println!("====================================");
    let path = Path::new("./inputs/day1.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day1::part_one(&numbers));
    println!("Part two: {:?}", day1::part_two(&numbers));
}

fn main() {
    day_1();
}
