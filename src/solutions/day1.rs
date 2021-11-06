use std::io::Error;

pub fn part_one(lines: Vec<String>) -> Result<(), Error> {
    for l in lines {
        println!("{}", l);
    }
    Ok(())
}

pub fn part_two(numbers: Vec<i64>) -> Result<(), Error> {
    for l in numbers {
        println!("{}", l);
    }
    Ok(())
}

#[test]
fn test_part_one() {}

#[test]
fn test_part_two() {}

#[bench]
fn bench_part_one(b: &mut test::Bencher) {
    let lines = super::read_lines(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        part_one(lines.clone()).unwrap();
    });
}

#[bench]
fn bench_part_two(b: &mut test::Bencher) {
    let numbers = super::read_ints(std::path::Path::new("./inputs/day1.txt")).unwrap();
    b.iter(|| {
        part_two(numbers.clone()).unwrap();
    });
}
