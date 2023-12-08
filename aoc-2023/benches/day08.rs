use aoc_2023::solutions::day08;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day08::part_one(divan::black_box(include_str!("../inputs/day08.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day08::part_two(divan::black_box(include_str!("../inputs/day08.txt",))).unwrap();
}
