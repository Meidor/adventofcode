#![feature(test)]
extern crate adventofcode;
extern crate test;

use adventofcode::{helpers, solutions::*};
use std::{collections::HashMap, env, path::Path};

fn day01() {
    println!("DAY01");
    println!("====================================");
    let path = Path::new("./inputs/day01.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day01::part_one(&lines));
    println!("Part two: {:?}", day01::part_two(&lines));
}

fn day02() {
    println!("DAY02");
    println!("====================================");
    let path = Path::new("./inputs/day02.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day02::part_one(&lines));
    println!("Part two: {:?}", day02::part_two(&lines));
}

fn day03() {
    println!("DAY03");
    println!("====================================");
    let path = Path::new("./inputs/day03.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day03::part_one(&lines));
    println!("Part two: {:?}", day03::part_two(&lines));
}

fn day04() {
    println!("DAY04");
    println!("====================================");
    let path = Path::new("./inputs/day04.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day04::part_one(&lines));
    println!("Part two: {:?}", day04::part_two(&lines));
}

fn day05() {
    println!("DAY05");
    println!("====================================");
    let path = Path::new("./inputs/day05.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day05::part_one(&lines));
    println!("Part two: {:?}", day05::part_two(&lines));
}

fn day06() {
    println!("DAY06");
    println!("====================================");
    let path = Path::new("./inputs/day06.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day06::part_one(&lines));
    println!("Part two: {:?}", day06::part_two(&lines));
}

fn day07() {
    println!("DAY07");
    println!("====================================");
    let path = Path::new("./inputs/day07.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day07::part_one(&lines));
    println!("Part two: {:?}", day07::part_two(&lines));
}

fn day08() {
    println!("DAY08");
    println!("====================================");
    let path = Path::new("./inputs/day08.txt");
    let lines = helpers::read_lines(path).unwrap();
    println!("Part one: {:?}", day08::part_one(&lines));
    println!("Part two: {:?}", day08::part_two(&lines));
}

fn get_days() -> HashMap<String, fn()> {
    let mut days: HashMap<String, fn()> = HashMap::new();
    days.insert("day01".to_string(), day01);
    days.insert("day02".to_string(), day02);
    days.insert("day03".to_string(), day03);
    days.insert("day04".to_string(), day04);
    days.insert("day05".to_string(), day05);
    days.insert("day06".to_string(), day06);
    days.insert("day07".to_string(), day07);
    days.insert("day08".to_string(), day08);

    days
}

fn main() {
    println!();
    println!("AOC 2021");
    println!();

    let days = get_days();
    let mut keys: Vec<&String> = days.keys().collect();
    keys.sort_unstable();

    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        let day = &args[0];
        if day == "all" {
            for day in keys {
                days.get(day).unwrap()();
                println!();
            }
            return;
        }

        match days.get(day) {
            Some(day) => day(),
            _ => println!("{} is not implemented yet!", &args[0]),
        }
        return;
    }
    days.get(*keys.last().unwrap()).unwrap()();
}
