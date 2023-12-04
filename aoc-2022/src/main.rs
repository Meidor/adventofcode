#![feature(test)]
extern crate aoc_2022;
extern crate test;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use color_eyre::eyre::Result;
use aoc_2022::solutions::*;
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


fn day05() -> Result<()>{
    println!("## DAY05");
    println!();
    day05_part01()?;
    day05_part02()?;
    Ok(())
}

fn day05_part01() -> Result<()>{
    let input = include_str!("../inputs/day05.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day05::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day05_part02() -> Result<()>{
    let input = include_str!("../inputs/day05.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day05::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day06() -> Result<()>{
    println!("## DAY06");
    println!();
    day06_part01()?;
    day06_part02()?;
    Ok(())
}

fn day06_part01() -> Result<()>{
    let input = include_str!("../inputs/day06.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day06::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day06_part02() -> Result<()>{
    let input = include_str!("../inputs/day06.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day06::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day07() -> Result<()>{
    println!("## DAY07");
    println!();
    day07_part01()?;
    day07_part02()?;
    Ok(())
}

fn day07_part01() -> Result<()>{
    let input = include_str!("../inputs/day07.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day07::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day07_part02() -> Result<()>{
    let input = include_str!("../inputs/day07.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day07::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day08() -> Result<()>{
    println!("## DAY08");
    println!();
    day08_part01()?;
    day08_part02()?;
    Ok(())
}

fn day08_part01() -> Result<()>{
    let input = include_str!("../inputs/day08.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day08::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day08_part02() -> Result<()>{
    let input = include_str!("../inputs/day08.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day08::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day09() -> Result<()>{
    println!("## DAY09");
    println!();
    day09_part01()?;
    day09_part02()?;
    Ok(())
}

fn day09_part01() -> Result<()>{
    let input = include_str!("../inputs/day09.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day09::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day09_part02() -> Result<()>{
    let input = include_str!("../inputs/day09.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day09::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day10() -> Result<()>{
    println!("## DAY10");
    println!();
    day10_part01()?;
    day10_part02()?;
    Ok(())
}

fn day10_part01() -> Result<()>{
    let input = include_str!("../inputs/day10.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day10::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day10_part02() -> Result<()>{
    let input = include_str!("../inputs/day10.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day10::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day11() -> Result<()>{
    println!("## DAY11");
    println!();
    day11_part01()?;
    day11_part02()?;
    Ok(())
}

fn day11_part01() -> Result<()>{
    let input = include_str!("../inputs/day11.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day11::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day11_part02() -> Result<()>{
    let input = include_str!("../inputs/day11.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day11::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day12() -> Result<()>{
    println!("## DAY12");
    println!();
    day12_part01()?;
    day12_part02()?;
    Ok(())
}

fn day12_part01() -> Result<()>{
    let input = include_str!("../inputs/day12.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day12::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day12_part02() -> Result<()>{
    let input = include_str!("../inputs/day12.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day12::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day13() -> Result<()>{
    println!("## DAY13");
    println!();
    day13_part01()?;
    day13_part02()?;
    Ok(())
}

fn day13_part01() -> Result<()>{
    let input = include_str!("../inputs/day13.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day13::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day13_part02() -> Result<()>{
    let input = include_str!("../inputs/day13.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day13::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day14() -> Result<()>{
    println!("## DAY14");
    println!();
    day14_part01()?;
    day14_part02()?;
    Ok(())
}

fn day14_part01() -> Result<()>{
    let input = include_str!("../inputs/day14.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day14::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day14_part02() -> Result<()>{
    let input = include_str!("../inputs/day14.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day14::part_two(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}


fn day15() -> Result<()>{
    println!("## DAY15");
    println!();
    day15_part01()?;
    day15_part02()?;
    Ok(())
}

fn day15_part01() -> Result<()>{
    let input = include_str!("../inputs/day15.txt");
    let start = Instant::now();
    println!("part one:");
    println!("{}", day15::part_one(input)?);
    println!();
    let elapsed = start.elapsed();
    println!("took {}ms ({}us)  ", elapsed.as_millis(), elapsed.as_micros());
    println!();
    Ok(())
}

fn day15_part02() -> Result<()>{
    let input = include_str!("../inputs/day15.txt");
    let start = Instant::now();
    println!("part two:");
    println!("{}", day15::part_two(input)?);
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
    days.insert("day05".to_string(), day05);
    days.insert("day05_part01".to_string(), day05_part01);
    days.insert("day05_part02".to_string(), day05_part02);
    days.insert("day06".to_string(), day06);
    days.insert("day06_part01".to_string(), day06_part01);
    days.insert("day06_part02".to_string(), day06_part02);
    days.insert("day07".to_string(), day07);
    days.insert("day07_part01".to_string(), day07_part01);
    days.insert("day07_part02".to_string(), day07_part02);
    days.insert("day08".to_string(), day08);
    days.insert("day08_part01".to_string(), day08_part01);
    days.insert("day08_part02".to_string(), day08_part02);
    days.insert("day09".to_string(), day09);
    days.insert("day09_part01".to_string(), day09_part01);
    days.insert("day09_part02".to_string(), day09_part02);
    days.insert("day10".to_string(), day10);
    days.insert("day10_part01".to_string(), day10_part01);
    days.insert("day10_part02".to_string(), day10_part02);
    days.insert("day11".to_string(), day11);
    days.insert("day11_part01".to_string(), day11_part01);
    days.insert("day11_part02".to_string(), day11_part02);
    days.insert("day12".to_string(), day12);
    days.insert("day12_part01".to_string(), day12_part01);
    days.insert("day12_part02".to_string(), day12_part02);
    days.insert("day13".to_string(), day13);
    days.insert("day13_part01".to_string(), day13_part01);
    days.insert("day13_part02".to_string(), day13_part02);
    days.insert("day14".to_string(), day14);
    days.insert("day14_part01".to_string(), day14_part01);
    days.insert("day14_part02".to_string(), day14_part02);
    days.insert("day15".to_string(), day15);
    days.insert("day15_part01".to_string(), day15_part01);
    days.insert("day15_part02".to_string(), day15_part02);
    
    Ok(days)
}

#[tracing::instrument]
fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    color_eyre::install()?;
    println!("# AOC 2022");
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
