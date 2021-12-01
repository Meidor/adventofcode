use adventofcode::solutions::{day18, read_ints};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day18.txt")).unwrap();
    c.bench_function("day 18 part 1", |b| b.iter(|| day18::part_one(black_box(&numbers))));
}

fn bench_part_two(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day18.txt")).unwrap();
    c.bench_function("day 18 part 2", |b| b.iter(||day18::part_two(black_box(&numbers))));
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);