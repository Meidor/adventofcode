use aoc_2023::solutions::day01;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day01::part_one(divan::black_box(include_str!("../inputs/day01.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    day01::part_two(divan::black_box(include_str!("../inputs/day01.txt",))).unwrap();
}
