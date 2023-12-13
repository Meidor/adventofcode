use aoc_2023::solutions::day13;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day13::part_one(divan::black_box(include_str!("../inputs/day13.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day13::part_two(divan::black_box(include_str!("../inputs/day13.txt",))).unwrap();
}
