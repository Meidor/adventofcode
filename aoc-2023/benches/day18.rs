use aoc_2023::solutions::day18;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day18::part_one(divan::black_box(include_str!("../inputs/day18.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day18::part_two(divan::black_box(include_str!("../inputs/day18.txt",))).unwrap();
}
