use adventofcode::solutions::{day01, read_lines};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_1", |b| b.iter(|| day01::part_one(black_box(&lines))));
}

fn bench_part_two(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_2", |b| b.iter(||day01::part_two(black_box(&lines))));
}

fn bench_part_one_old_school(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_1_old_school", |b| b.iter(||day01::part_one_old_school(black_box(&lines))));
}

fn bench_part_two_old_school(c: &mut Criterion) {
    let lines = read_lines(std::path::Path::new("./inputs/day01.txt")).unwrap();
    c.bench_function("day01_part_2_old_school", |b| b.iter(|| day01::part_two_old_school(black_box(&lines))));
}

criterion_group!(functional, bench_part_one, bench_part_two);
criterion_group!(loops, bench_part_one_old_school, bench_part_two_old_school);
criterion_main!(functional, loops);