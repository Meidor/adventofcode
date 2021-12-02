#![feature(test)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate test;
extern crate adventofcode;

use std::{path::Path, collections::HashMap, env};
use adventofcode::solutions::{self, *};


fn day01(){
    println!("DAY 01");
    println!("====================================");
    let path = Path::new("./inputs/day01.txt");
    let lines = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day01::part_one(&lines));
    println!("Part two: {:?}", day01::part_two(&lines));
}

fn day02(){
    println!("DAY 02");
    println!("====================================");
    let path = Path::new("./inputs/day02.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day02::part_one(&lines));
    println!("Part two: {:?}", day02::part_two(&lines));
}

fn day03(){
    println!("DAY 03");
    println!("====================================");
    let path = Path::new("./inputs/day03.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day03::part_one(&lines));
    println!("Part two: {:?}", day03::part_two(&lines));
}

fn day04(){
    println!("DAY 04");
    println!("====================================");
    let path = Path::new("./inputs/day04.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day04::part_one(&lines));
    println!("Part two: {:?}", day04::part_two(&lines));
}

fn day05(){
    println!("DAY 05");
    println!("====================================");
    let path = Path::new("./inputs/day05.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day05::part_one(&lines));
    println!("Part two: {:?}", day05::part_two(&lines));
}

fn day06(){
    println!("DAY 06");
    println!("====================================");
    let path = Path::new("./inputs/day06.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day06::part_one(&lines));
    println!("Part two: {:?}", day06::part_two(&lines));
}

fn day07(){
    println!("DAY 07");
    println!("====================================");
    let path = Path::new("./inputs/day07.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day07::part_one(&lines));
    println!("Part two: {:?}", day07::part_two(&lines));
}

fn day08(){
    println!("DAY 08");
    println!("====================================");
    let path = Path::new("./inputs/day08.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day08::part_one(&lines));
    println!("Part two: {:?}", day08::part_two(&lines));
}

fn day09(){
    println!("DAY 09");
    println!("====================================");
    let path = Path::new("./inputs/day09.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day09::part_one(&lines));
    println!("Part two: {:?}", day09::part_two(&lines));
}

fn day10(){
    println!("DAY 10");
    println!("====================================");
    let path = Path::new("./inputs/day10.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day10::part_one(&lines));
    println!("Part two: {:?}", day10::part_two(&lines));
}

fn day11(){
    println!("DAY 11");
    println!("====================================");
    let path = Path::new("./inputs/day11.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day11::part_one(&lines));
    println!("Part two: {:?}", day11::part_two(&lines));
}

fn day12(){
    println!("DAY 12");
    println!("====================================");
    let path = Path::new("./inputs/day12.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day12::part_one(&lines));
    println!("Part two: {:?}", day12::part_two(&lines));
}

fn day13(){
    println!("DAY 13");
    println!("====================================");
    let path = Path::new("./inputs/day13.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day13::part_one(&lines));
    println!("Part two: {:?}", day13::part_two(&lines));
}

fn day14(){
    println!("DAY 14");
    println!("====================================");
    let path = Path::new("./inputs/day14.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day14::part_one(&lines));
    println!("Part two: {:?}", day14::part_two(&lines));
}

fn day15(){
    println!("DAY 15");
    println!("====================================");
    let path = Path::new("./inputs/day15.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day15::part_one(&lines));
    println!("Part two: {:?}", day15::part_two(&lines));
}

fn day16(){
    println!("DAY 16");
    println!("====================================");
    let path = Path::new("./inputs/day16.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day16::part_one(&lines));
    println!("Part two: {:?}", day16::part_two(&lines));
}

fn day17(){
    println!("DAY 17");
    println!("====================================");
    let path = Path::new("./inputs/day17.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day17::part_one(&lines));
    println!("Part two: {:?}", day17::part_two(&lines));
}

fn day18(){
    println!("DAY 18");
    println!("====================================");
    let path = Path::new("./inputs/day18.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day18::part_one(&lines));
    println!("Part two: {:?}", day18::part_two(&lines));
}

fn day19(){
    println!("DAY 19");
    println!("====================================");
    let path = Path::new("./inputs/day19.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day19::part_one(&lines));
    println!("Part two: {:?}", day19::part_two(&lines));
}

fn day20(){
    println!("DAY 20");
    println!("====================================");
    let path = Path::new("./inputs/day20.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day20::part_one(&lines));
    println!("Part two: {:?}", day20::part_two(&lines));
}

fn day21(){
    println!("DAY 21");
    println!("====================================");
    let path = Path::new("./inputs/day21.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day21::part_one(&lines));
    println!("Part two: {:?}", day21::part_two(&lines));
}

fn day22(){
    println!("DAY 22");
    println!("====================================");
    let path = Path::new("./inputs/day22.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day22::part_one(&lines));
    println!("Part two: {:?}", day22::part_two(&lines));
}

fn day23(){
    println!("DAY 23");
    println!("====================================");
    let path = Path::new("./inputs/day23.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day23::part_one(&lines));
    println!("Part two: {:?}", day23::part_two(&lines));
}

fn day24(){
    println!("DAY 24");
    println!("====================================");
    let path = Path::new("./inputs/day24.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day24::part_one(&lines));
    println!("Part two: {:?}", day24::part_two(&lines));
}

fn day25(){
    println!("DAY 25");
    println!("====================================");
    let path = Path::new("./inputs/day25.txt");
    let lines = solutions::read_lines(path).unwrap();
    println!("Part one: {:?}", day25::part_one(&lines));
    println!("Part two: {:?}", day25::part_two(&lines));
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
    days.insert("day16".to_string(), day16);
    days.insert("day17".to_string(), day17);
    days.insert("day18".to_string(), day18);
    days.insert("day19".to_string(), day19);
    days.insert("day20".to_string(), day20);
    days.insert("day21".to_string(), day21);
    days.insert("day22".to_string(), day22);
    days.insert("day23".to_string(), day23);
    days.insert("day24".to_string(), day24);
    days.insert("day25".to_string(), day25);
    
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
    if args.len() > 0 {
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
            _ => println!("{} is not implemented yet!", &args[0])
        }
        return;
    }
    days.get(*keys.last().unwrap()).unwrap()();
}
