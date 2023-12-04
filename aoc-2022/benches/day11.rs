use adventofcode::solutions::day11;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day11::part_one(divan::black_box(include_str!("../inputs/day11.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day11::part_two(divan::black_box(include_str!("../inputs/day11.txt",))).unwrap();
}
