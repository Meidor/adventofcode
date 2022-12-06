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

fn day02() -> Result<()>{
    println!("## DAY02");
    println!();
    let input = include_str!("../inputs/day02.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day02::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day02::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day03() -> Result<()>{
    println!("## DAY03");
    println!();
    let input = include_str!("../inputs/day03.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day03::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day03::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day04() -> Result<()>{
    println!("## DAY04");
    println!();
    let input = include_str!("../inputs/day04.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day04::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day04::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day05() -> Result<()>{
    println!("## DAY05");
    println!();
    let input = include_str!("../inputs/day05.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day05::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day05::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day06() -> Result<()>{
    println!("## DAY06");
    println!();
    let input = include_str!("../inputs/day06.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day06::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day06::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn get_days() -> Result<HashMap<String, fn() -> Result<()>>> {
    let mut days: HashMap<String, fn() -> Result<()>> = HashMap::new();
    days.insert("day01".to_string(), day01);
    days.insert("day02".to_string(), day02);
    days.insert("day03".to_string(), day03);
    days.insert("day04".to_string(), day04);
    days.insert("day05".to_string(), day05);
    days.insert("day06".to_string(), day06);
    
    Ok(days)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("# AOC 2022");
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
                days.get(day).expect(&format!("day {} not found", day))()?;
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
