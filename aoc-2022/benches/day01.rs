use adventofcode::solutions::day01;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day01::part_one(divan::black_box(include_str!("../inputs/day01.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day01::part_two(divan::black_box(include_str!("../inputs/day01.txt",))).unwrap();
}
