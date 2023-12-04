use adventofcode::solutions::day12;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day12::part_one(divan::black_box(include_str!("../inputs/day12.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day12::part_two(divan::black_box(include_str!("../inputs/day12.txt",))).unwrap();
}
