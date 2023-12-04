use adventofcode::solutions::day05;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day05::part_one(divan::black_box(include_str!("../inputs/day05.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day05::part_two(divan::black_box(include_str!("../inputs/day05.txt",))).unwrap();
}
