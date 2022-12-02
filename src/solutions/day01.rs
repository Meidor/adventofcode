#[inline]
pub fn part_one(input: &str) -> String {
    let mut current: u64 = 0;
    let mut max: u64 = 0;
    for line in input.lines() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }
        current += line.parse::<u64>().unwrap();
    }
    max.to_string()
}

#[inline]
pub fn part_two(input: &str) -> String {
    let mut sums: Vec<u64> = Vec::new();
    let mut current: u64 = 0;
    for line in input.lines() {
        if line == "" {
            sums.push(current);
            current = 0;
            continue;
        }
        current += u64::from_str_radix(line, 10).expect("couldn't parse string as u64");
    }
    sums.sort_unstable();
    sums[sums.len() - 3..sums.len()]
        .into_iter()
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
    }

    #[test]
    fn test_part_one() {
        let expected = "24000";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "45000";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
