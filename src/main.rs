#![feature(test)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate test;
extern crate adventofcode;

use std::{path::Path, collections::HashMap, env};
use adventofcode::solutions::{self, *};


fn day1(){
    println!("DAY 1");
    println!("====================================");
    let path = Path::new("./inputs/day1.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day1::part_one(&numbers));
    println!("Part two: {:?}", day1::part_two(&numbers));
}

fn day2(){
    println!("DAY 2");
    println!("====================================");
    let path = Path::new("./inputs/day2.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day2::part_one(&numbers));
    println!("Part two: {:?}", day2::part_two(&numbers));
}

fn day3(){
    println!("DAY 3");
    println!("====================================");
    let path = Path::new("./inputs/day3.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day3::part_one(&numbers));
    println!("Part two: {:?}", day3::part_two(&numbers));
}

fn day4(){
    println!("DAY 4");
    println!("====================================");
    let path = Path::new("./inputs/day4.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day4::part_one(&numbers));
    println!("Part two: {:?}", day4::part_two(&numbers));
}

fn day5(){
    println!("DAY 5");
    println!("====================================");
    let path = Path::new("./inputs/day5.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day5::part_one(&numbers));
    println!("Part two: {:?}", day5::part_two(&numbers));
}

fn day6(){
    println!("DAY 6");
    println!("====================================");
    let path = Path::new("./inputs/day6.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day6::part_one(&numbers));
    println!("Part two: {:?}", day6::part_two(&numbers));
}

fn day7(){
    println!("DAY 7");
    println!("====================================");
    let path = Path::new("./inputs/day7.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day7::part_one(&numbers));
    println!("Part two: {:?}", day7::part_two(&numbers));
}

fn day8(){
    println!("DAY 8");
    println!("====================================");
    let path = Path::new("./inputs/day8.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day8::part_one(&numbers));
    println!("Part two: {:?}", day8::part_two(&numbers));
}

fn day9(){
    println!("DAY 9");
    println!("====================================");
    let path = Path::new("./inputs/day9.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day9::part_one(&numbers));
    println!("Part two: {:?}", day9::part_two(&numbers));
}

fn day10(){
    println!("DAY 10");
    println!("====================================");
    let path = Path::new("./inputs/day10.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day10::part_one(&numbers));
    println!("Part two: {:?}", day10::part_two(&numbers));
}

fn day11(){
    println!("DAY 11");
    println!("====================================");
    let path = Path::new("./inputs/day11.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day11::part_one(&numbers));
    println!("Part two: {:?}", day11::part_two(&numbers));
}

fn day12(){
    println!("DAY 12");
    println!("====================================");
    let path = Path::new("./inputs/day12.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day12::part_one(&numbers));
    println!("Part two: {:?}", day12::part_two(&numbers));
}

fn day13(){
    println!("DAY 13");
    println!("====================================");
    let path = Path::new("./inputs/day13.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day13::part_one(&numbers));
    println!("Part two: {:?}", day13::part_two(&numbers));
}

fn day14(){
    println!("DAY 14");
    println!("====================================");
    let path = Path::new("./inputs/day14.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day14::part_one(&numbers));
    println!("Part two: {:?}", day14::part_two(&numbers));
}

fn day15(){
    println!("DAY 15");
    println!("====================================");
    let path = Path::new("./inputs/day15.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day15::part_one(&numbers));
    println!("Part two: {:?}", day15::part_two(&numbers));
}

fn day16(){
    println!("DAY 16");
    println!("====================================");
    let path = Path::new("./inputs/day16.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day16::part_one(&numbers));
    println!("Part two: {:?}", day16::part_two(&numbers));
}

fn day17(){
    println!("DAY 17");
    println!("====================================");
    let path = Path::new("./inputs/day17.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day17::part_one(&numbers));
    println!("Part two: {:?}", day17::part_two(&numbers));
}

fn day18(){
    println!("DAY 18");
    println!("====================================");
    let path = Path::new("./inputs/day18.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day18::part_one(&numbers));
    println!("Part two: {:?}", day18::part_two(&numbers));
}

fn day19(){
    println!("DAY 19");
    println!("====================================");
    let path = Path::new("./inputs/day19.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day19::part_one(&numbers));
    println!("Part two: {:?}", day19::part_two(&numbers));
}

fn day20(){
    println!("DAY 20");
    println!("====================================");
    let path = Path::new("./inputs/day20.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day20::part_one(&numbers));
    println!("Part two: {:?}", day20::part_two(&numbers));
}

fn day21(){
    println!("DAY 21");
    println!("====================================");
    let path = Path::new("./inputs/day21.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day21::part_one(&numbers));
    println!("Part two: {:?}", day21::part_two(&numbers));
}

fn day22(){
    println!("DAY 22");
    println!("====================================");
    let path = Path::new("./inputs/day22.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day22::part_one(&numbers));
    println!("Part two: {:?}", day22::part_two(&numbers));
}

fn day23(){
    println!("DAY 23");
    println!("====================================");
    let path = Path::new("./inputs/day23.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day23::part_one(&numbers));
    println!("Part two: {:?}", day23::part_two(&numbers));
}

fn day24(){
    println!("DAY 24");
    println!("====================================");
    let path = Path::new("./inputs/day24.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day24::part_one(&numbers));
    println!("Part two: {:?}", day24::part_two(&numbers));
}

fn day25(){
    println!("DAY 25");
    println!("====================================");
    let path = Path::new("./inputs/day25.txt");
    let numbers = solutions::read_ints(path).unwrap();
    println!("Part one: {:?}", day25::part_one(&numbers));
    println!("Part two: {:?}", day25::part_two(&numbers));
}


fn get_days() -> HashMap<String, fn()> {
    let mut days: HashMap<String, fn()> = HashMap::new();
    days.insert("day1".to_string(), day1);
    days.insert("day2".to_string(), day2);
    days.insert("day3".to_string(), day3);
    days.insert("day4".to_string(), day4);
    days.insert("day5".to_string(), day5);
    days.insert("day6".to_string(), day6);
    days.insert("day7".to_string(), day7);
    days.insert("day8".to_string(), day8);
    days.insert("day9".to_string(), day9);
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
