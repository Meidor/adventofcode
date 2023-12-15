use aoc_2023::solutions::day14;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day14::part_one(divan::black_box(include_str!("../inputs/day14.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day14::part_two(divan::black_box(include_str!("../inputs/day14.txt",))).unwrap();
}
