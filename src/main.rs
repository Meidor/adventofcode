#![feature(test)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate test;

use std::path::Path;
use solutions::*;

mod solutions;

fn day_1(){
    println!("DAY 1");
    println!("====================================");
    let path = Path::new("./inputs/day1.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day1::part_one(&numbers));
    println!("Part two: {:?}", day1::part_two(&numbers));
}

fn day_2() {
    println!("DAY 2");
    println!("====================================");
    let path = Path::new("./inputs/day2.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day2::part_one(&numbers));
    println!("Part two: {:?}", day2::part_two(&numbers));
}

fn main() {
    day_1();
}
