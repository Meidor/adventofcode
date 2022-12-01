#[inline]
pub fn part_one(lines: &[String]) -> String {
    let mut current: u64 = 0;
    let mut max: u64 = 0;
    for line in lines {
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
pub fn part_two(lines: &[String]) -> String {
    let mut sums: Vec<u64> = Vec::new();
    let mut current: u64 = 0;
    for line in lines {
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

    fn test_input() -> Vec<String> {
        vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
            "".to_string(),
        ]
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
