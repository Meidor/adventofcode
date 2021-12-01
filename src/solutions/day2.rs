pub fn part_one(numbers: &Vec<i64>) -> usize {
    todo!()
}

pub fn part_two(numbers: &Vec<i64>) -> usize {
    todo!()
}

fn test_input() -> Vec<i64> {
    vec![]
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
        // part_one(&numbers);
    });
}

#[bench]
fn bench_part_two(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        // part_two(&numbers);
    });
}
