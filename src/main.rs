#![feature(test)]
extern crate adventofcode;
extern crate test;

use color_eyre::eyre::Result;
use adventofcode::solutions::*;
use std::{collections::HashMap, env, time::Instant};

fn day01() -> Result<()>{
    println!("## DAY01");
    println!();
    let input = include_str!("../inputs/day01.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day01::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day01::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


type Days = HashMap<String, fn() -> Result<()>>;

fn get_days() -> Result<Days> {
    let mut days: Days = HashMap::new();
    days.insert("day01".to_string(), day01);
    
    Ok(days)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("# AOC 2023");
    println!();

    let days = get_days()?;
    let mut keys: Vec<&String> = days.keys().collect();
    keys.sort_unstable();
    let args: Vec<String> = env::args().skip(1).collect();

    let start = Instant::now();
    if !args.is_empty() {
        let day = &args[0];
        if day == "all" {
            for day in keys {
                days.get(day).unwrap_or_else(|| panic!("day {} not found", day))()?;
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
                Some(day) => day()?,
                _ => println!("{} is not implemented yet!", &args[0]),
            }
        }
    } else {
        days.get(*keys.last().expect("no days in HashMap")).expect("day not found")()?;
    }
    Ok(())
}
