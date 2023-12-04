use adventofcode::solutions::day06;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day06::part_one(divan::black_box(include_str!("../inputs/day06.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day06::part_two(divan::black_box(include_str!("../inputs/day06.txt",))).unwrap();
}
