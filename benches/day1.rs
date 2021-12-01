use adventofcode::solutions::{day1, read_ints};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    c.bench_function("day 1 part 1", |b| b.iter(|| day1::part_one(black_box(&numbers))));
}

fn bench_part_two(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    c.bench_function("day 1 part 2", |b| b.iter(||day1::part_two(black_box(&numbers))));
}

fn bench_part_one_old_school(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    c.bench_function("day 1 part 1 old school", |b| b.iter(||day1::part_one_old_school(black_box(&numbers))));
}

fn bench_part_two_old_school(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    c.bench_function("day 1 part 2 old school", |b| b.iter(|| day1::part_two_old_school(black_box(&numbers))));
}

criterion_group!(functional, bench_part_one, bench_part_two);
criterion_group!(loops, bench_part_one_old_school, bench_part_two_old_school);
criterion_main!(functional, loops);