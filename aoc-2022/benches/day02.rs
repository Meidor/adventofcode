use aoc_2022::solutions::day02;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day02::part_one(divan::black_box(include_str!("../inputs/day02.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day02::part_two(divan::black_box(include_str!("../inputs/day02.txt",))).unwrap();
}
