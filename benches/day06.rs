use adventofcode::solutions::{day06, read_ints};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_part_one(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day06.txt")).unwrap();
    c.bench_function("day06_part_1", |b| b.iter(|| day06::part_one(black_box(&numbers))));
}

fn bench_part_two(c: &mut Criterion) {
    let numbers = read_ints(std::path::Path::new("./inputs/day06.txt")).unwrap();
    c.bench_function("day06_part_2", |b| b.iter(||day06::part_two(black_box(&numbers))));
}

criterion_group!(benches, bench_part_one, bench_part_two);
criterion_main!(benches);