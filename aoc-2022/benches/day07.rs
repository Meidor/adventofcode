use adventofcode::solutions::day07;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part01() {
    day07::part_one(divan::black_box(include_str!("../inputs/day07.txt",))).unwrap();
}

#[divan::bench]
fn part02() {
    day07::part_two(divan::black_box(include_str!("../inputs/day07.txt",))).unwrap();
}
