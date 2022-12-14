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

fn day07() -> Result<()>{
    println!("## DAY07");
    println!();
    let input = include_str!("../inputs/day07.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day07::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day07::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day08() -> Result<()>{
    println!("## DAY08");
    println!();
    let input = include_str!("../inputs/day08.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day08::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day08::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day09() -> Result<()>{
    println!("## DAY09");
    println!();
    let input = include_str!("../inputs/day09.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day09::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day09::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day10() -> Result<()>{
    println!("## DAY10");
    println!();
    let input = include_str!("../inputs/day10.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day10::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day10::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day11() -> Result<()>{
    println!("## DAY11");
    println!();
    let input = include_str!("../inputs/day11.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day11::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day11::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day12() -> Result<()>{
    println!("## DAY12");
    println!();
    let input = include_str!("../inputs/day12.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day12::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day12::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day13() -> Result<()>{
    println!("## DAY13");
    println!();
    let input = include_str!("../inputs/day13.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day13::part_one(input)?);
    println!();
    println!("part two:");
    println!("{}", day13::part_two(input)?);
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
    days.insert("day07".to_string(), day07);
    days.insert("day08".to_string(), day08);
    days.insert("day09".to_string(), day09);
    days.insert("day10".to_string(), day10);
    days.insert("day11".to_string(), day11);
    days.insert("day12".to_string(), day12);
    days.insert("day13".to_string(), day13);
    
    Ok(days)
}

fn main() -> Result<()> {
    env_logger::init();
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
