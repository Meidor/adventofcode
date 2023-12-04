use adventofcode::solutions::day15;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day15::part_one(divan::black_box(include_str!("../inputs/day15.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day15::part_two(divan::black_box(include_str!("../inputs/day15.txt",))).unwrap();
}
