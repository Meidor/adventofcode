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
        solve(numbers.windows(4))
}

pub fn part_one_old_school(numbers: &Vec<i64>) -> usize {
    let mut previous = i64::MAX;
    let mut count: usize = 0;
    for i in numbers {
        if *i > previous {
            count += 1;
        }
        previous = *i;
    }
    count
}

pub fn part_two_old_school(numbers: &Vec<i64>) -> usize {
    let mut previous = i64::MAX;
    let mut count: usize = 0;
    for i in 0..(numbers.len() - 2) {
        let sum = numbers[i] + numbers[i + 1] + numbers[i + 2];
        if sum > previous {
            count += 1;
        }
        previous = sum;
    }
    count
}

fn test_input() -> Vec<i64> {
    vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
}

#[test]
fn test_part_one() {
    let expected = 7;
    let actual = part_one(&test_input());
    assert_eq!(expected, actual);
}

#[test]
fn test_part_two() {
    let expected = 5;
    let actual = part_two(&test_input());
    assert_eq!(expected, actual);
}

#[bench]
fn bench_part_one(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        let expected = 1462;
        let actual = part_one(&numbers);
        assert_eq!(expected, actual);
    });
}

#[bench]
fn bench_part_one_old_school(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        let expected = 1462;
        let actual = part_one_old_school(&numbers);
        assert_eq!(expected, actual);
    });
}

#[bench]
fn bench_part_two(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        let expected = 1497;
        let actual = part_two(&numbers);
        assert_eq!(expected, actual);
    });
}

#[bench]
fn bench_part_two_old_school(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        let expected = 1497;
        let actual = part_two_old_school(&numbers);
        assert_eq!(expected, actual);
    });
}
