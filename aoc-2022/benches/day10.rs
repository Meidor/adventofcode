use adventofcode::solutions::day10;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day10::part_one(divan::black_box(include_str!("../inputs/day10.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day10::part_two(divan::black_box(include_str!("../inputs/day10.txt",))).unwrap();
}
