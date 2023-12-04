#![feature(test)]
extern crate aoc_{{ year }};
extern crate test;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use color_eyre::eyre::Result;
use aoc_{{ year }}::solutions::*;
use std::{collections::HashMap, env, time::Instant};

type Days = HashMap<String, fn() -> Result<()>>;

fn get_days() -> Result<Days> {
    let mut days: Days = HashMap::new();
    Ok(days)
}

#[tracing::instrument]
fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt::init();

    color_eyre::install()?;
    println!("# AOC {{ year }}");
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
