#![feature(test)]
extern crate adventofcode;
extern crate test;

use adventofcode::{helpers, solutions::*};
use std::{collections::HashMap, env, path::Path, time::Instant};

fn day01() {
    println!("## DAY01");
    println!();
    let path = Path::new("./inputs/day01.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day01::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day01::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day02() {
    println!("## DAY02");
    println!();
    let path = Path::new("./inputs/day02.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day02::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day02::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day03() {
    println!("## DAY03");
    println!();
    let path = Path::new("./inputs/day03.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day03::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day03::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day04() {
    println!("## DAY04");
    println!();
    let path = Path::new("./inputs/day04.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day04::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day04::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day05() {
    println!("## DAY05");
    println!();
    let path = Path::new("./inputs/day05.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day05::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day05::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day06() {
    println!("## DAY06");
    println!();
    let path = Path::new("./inputs/day06.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day06::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day06::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day07() {
    println!("## DAY07");
    println!();
    let path = Path::new("./inputs/day07.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day07::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day07::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day08() {
    println!("## DAY08");
    println!();
    let path = Path::new("./inputs/day08.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day08::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day08::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day09() {
    println!("## DAY09");
    println!();
    let path = Path::new("./inputs/day09.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day09::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day09::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day10() {
    println!("## DAY10");
    println!();
    let path = Path::new("./inputs/day10.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day10::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day10::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day11() {
    println!("## DAY11");
    println!();
    let path = Path::new("./inputs/day11.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day11::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day11::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day12() {
    println!("## DAY12");
    println!();
    let path = Path::new("./inputs/day12.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day12::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day12::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day13() {
    println!("## DAY13");
    println!();
    let path = Path::new("./inputs/day13.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day13::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day13::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day14() {
    println!("## DAY14");
    println!();
    let path = Path::new("./inputs/day14.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day14::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day14::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
}

fn day15() {
    println!("## DAY15");
    println!();
    let path = Path::new("./inputs/day15.txt");
    let lines = helpers::read_lines(path).unwrap();
    let start = Instant::now();
    println!("part one:");
    println!("{}", day15::part_one(&lines));
    println!();
    println!("part two:");
    println!("{}", day15::part_two(&lines));
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
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
    days.insert("day09".to_string(), day09);
    days.insert("day10".to_string(), day10);
    days.insert("day11".to_string(), day11);
    days.insert("day12".to_string(), day12);
    days.insert("day13".to_string(), day13);
    days.insert("day14".to_string(), day14);
    days.insert("day15".to_string(), day15);
    
    days
}

fn main() {
    println!("# AOC 2021");
    println!();

    let days = get_days();
    let mut keys: Vec<&String> = days.keys().collect();
    keys.sort_unstable();
    let args: Vec<String> = env::args().skip(1).collect();

    let start = Instant::now();
    if !args.is_empty() {
        let day = &args[0];
        if day == "all" {
            for day in keys {
                days.get(day).unwrap()();
            }
            let elapsed = start.elapsed();
            println!("## Total execution time");
            println!();
            println!(
                "took {}ms ({}us)  ",
                elapsed.as_millis(),
                elapsed.as_micros()
            );
        } else {
            match days.get(day) {
                Some(day) => day(),
                _ => println!("{} is not implemented yet!", &args[0]),
            }
        }
    } else {
        days.get(*keys.last().unwrap()).unwrap()();
    }
}
