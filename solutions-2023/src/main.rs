#![feature(test)]
extern crate solutions_2023;
extern crate test;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use color_eyre::eyre::Result;
use solutions_2023::solutions::*;
use std::{collections::HashMap, env, time::Instant};


fn day01() -> Result<()>{
    println!("## DAY01");
    println!();
    day01_part01()?;
    day01_part02()?;
    Ok(())
}

fn day01_part01() -> Result<()>{
    let input = include_str!("../inputs/day01.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day01::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day01_part02() -> Result<()>{
    let input = include_str!("../inputs/day01.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day01::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day02() -> Result<()>{
    println!("## DAY02");
    println!();
    day02_part01()?;
    day02_part02()?;
    Ok(())
}

fn day02_part01() -> Result<()>{
    let input = include_str!("../inputs/day02.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day02::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day02_part02() -> Result<()>{
    let input = include_str!("../inputs/day02.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day02::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day03() -> Result<()>{
    println!("## DAY03");
    println!();
    day03_part01()?;
    day03_part02()?;
    Ok(())
}

fn day03_part01() -> Result<()>{
    let input = include_str!("../inputs/day03.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day03::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day03_part02() -> Result<()>{
    let input = include_str!("../inputs/day03.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day03::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day04() -> Result<()>{
    println!("## DAY04");
    println!();
    day04_part01()?;
    day04_part02()?;
    Ok(())
}

fn day04_part01() -> Result<()>{
    let input = include_str!("../inputs/day04.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day04::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day04_part02() -> Result<()>{
    let input = include_str!("../inputs/day04.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day04::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


type Days = HashMap<String, fn() -> Result<()>>;

fn get_days() -> Result<Days> {
    let mut days: Days = HashMap::new();
    days.insert("day01".to_string(), day01);
    days.insert("day01_part01".to_string(), day01_part01);
    days.insert("day01_part02".to_string(), day01_part02);
    days.insert("day02".to_string(), day02);
    days.insert("day02_part01".to_string(), day02_part01);
    days.insert("day02_part02".to_string(), day02_part02);
    days.insert("day03".to_string(), day03);
    days.insert("day03_part01".to_string(), day03_part01);
    days.insert("day03_part02".to_string(), day03_part02);
    days.insert("day04".to_string(), day04);
    days.insert("day04_part01".to_string(), day04_part01);
    days.insert("day04_part02".to_string(), day04_part02);
    
    Ok(days)
}

#[tracing::instrument]
fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    color_eyre::install()?;
    println!("# AOC 2023");
    println!();

    let days = get_days()?;
    let mut keys: Vec<&String> = days.keys().filter(|k| !k.contains("part")).collect();

    keys.sort_unstable();
    let args: Vec<String> = env::args().skip(1).collect();

    let start = Instant::now();

    if args.len() == 2 {
        let day = &args[0];
        let part = &args[1];
        println!("## {}", day.to_uppercase());
        days.get(&format!("{}_{}", day, part))
            .unwrap_or_else(|| panic!("day {} part {} not found", day, part))()?;
        return Ok(());
    }

    if args.len() == 1 {
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
        return Ok(());
    }

    days.get(*keys.last().expect("no days in HashMap")).expect("day not found")()?;
    Ok(())
}
