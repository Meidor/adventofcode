use aoc_2022::solutions::day04;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day04::part_one(divan::black_box(include_str!("../inputs/day04.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day04::part_two(divan::black_box(include_str!("../inputs/day04.txt",))).unwrap();
}
