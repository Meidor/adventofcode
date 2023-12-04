use adventofcode::solutions::day09;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day09::part_one(divan::black_box(include_str!("../inputs/day09.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day09::part_two(divan::black_box(include_str!("../inputs/day09.txt",))).unwrap();
}
