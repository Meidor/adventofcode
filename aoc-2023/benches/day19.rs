use aoc_2023::solutions::day19;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day19::part_one(divan::black_box(include_str!("../inputs/day19.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day19::part_two(divan::black_box(include_str!("../inputs/day19.txt",))).unwrap();
}
