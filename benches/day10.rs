use adventofcode::solutions::day10;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let input = include_str!("../inputs/day10.txt");
    c.bench_function("day10_part_1", |b| b.iter(|| day10::part_one(black_box(input))));
}

fn bench_part_two(c: &mut Criterion) {
    let input = include_str!("../inputs/day10.txt");
    c.bench_function("day10_part_2", |b| b.iter(||day10::part_two(black_box(input))));
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);