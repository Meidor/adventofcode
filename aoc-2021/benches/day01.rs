use aoc_2021::helpers::read_lines;
use aoc_2021::solutions::day01;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_1", |b| {
        b.iter(|| day01::part_one(black_box(&lines)))
    });
}

fn bench_part_two(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_2", |b| {
        b.iter(|| day01::part_two(black_box(&lines)))
    });
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);
