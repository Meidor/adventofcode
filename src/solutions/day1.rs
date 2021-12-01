use std::slice::Windows;

fn solve(windows: Windows<i64>) -> usize {
    windows
        .filter(|x| x.first() < x.last())
        .count()
}

pub fn part_one(numbers: &Vec<i64>) -> usize {
        solve(numbers.windows(2))
}

pub fn part_two(numbers: &Vec<i64>) -> usize {
        solve(numbers.windows(3))
}



fn test_input() -> Vec<i64> {
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
}

#[test]
fn test_part_one() {
    let actual = part_one(&test_input());
    assert_eq!(7, actual);
}

#[test]
fn test_part_two() {
    let actual = part_two(&test_input());
    assert_eq!(5, actual);
}

#[bench]
fn bench_part_one(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        part_one(&numbers);
    });
}

#[bench]
fn bench_part_two(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        part_two(&numbers);
    });
}
