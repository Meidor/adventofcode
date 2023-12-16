use aoc_2023::solutions::day16;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day16::part_one(divan::black_box(include_str!("../inputs/day16.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day16::part_two(divan::black_box(include_str!("../inputs/day16.txt",))).unwrap();
}
