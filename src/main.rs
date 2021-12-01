#![feature(test)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate test;

use std::{path::Path, collections::HashMap, env};
use solutions::*;

mod solutions;

fn day1(){
    println!("DAY 1");
    println!("====================================");
    let path = Path::new("./inputs/day1.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day1::part_one(&numbers));
    println!("Part two: {:?}", day1::part_two(&numbers));
}

fn day2() {
    println!("DAY 2");
    println!("====================================");
    let path = Path::new("./inputs/day2.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day2::part_one(&numbers));
    println!("Part two: {:?}", day2::part_two(&numbers));
}

fn get_days() -> HashMap<String, fn()> {
    let mut days: HashMap<String, fn()> = HashMap::new();
    days.insert("day1".to_string(), day1);
    days.insert("day2".to_string(), day2);
    days
}

fn main() {
    println!();
    println!("AOC 2021");
    println!();
    
    let days = get_days();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        let day = &args[0];
        if day == "all" {
            for day in days.values() {
                day();
                println!();
            }
            return;
        }
        
        match days.get(day) {
            Some(day) => day(),
            _ => println!("{} is not implemented yet!", &args[0])
        }
        return;
    }
        
    let mut keys: Vec<&String> = days.keys().collect();
    keys.sort_unstable();
    days.get(*keys.last().unwrap()).unwrap()();
}
