use aoc_2022::solutions::day03;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day03::part_one(divan::black_box(include_str!("../inputs/day03.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day03::part_two(divan::black_box(include_str!("../inputs/day03.txt",))).unwrap();
}
