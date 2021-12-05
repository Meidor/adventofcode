fn get_bitcount(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    let strlen = lines[0].len();
    let mut i: usize = 0;
    let mut zeroes = vec![0; strlen];
    let mut ones = vec![0; strlen];
    for line in lines {
        for char in &mut line.chars() {
            match char {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => {}
            }
            i += 1;
        }
        i = 0;
    }
    (zeroes, ones)
}

fn filter_input(lines: Vec<String>, i: usize, keep_most: bool) -> Vec<String> {
    if lines.len() == 1 {
        return lines;
    }
    let (zeroes, ones) = get_bitcount(&lines);
    let flag = if zeroes[i] > ones[i] {
        keep_most
    } else {
        !keep_most
    };

    if flag {
        lines
            .into_iter()
            .filter(|x| x.get(i..i + 1).unwrap() == "0")
            .collect()
    } else {
        lines
            .into_iter()
            .filter(|x| x.get(i..i + 1).unwrap() == "1")
            .collect()
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let strlen = lines[0].len();
    let (zeroes, ones) = get_bitcount(lines);
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for i in 0..strlen {
        let z = zeroes[i];
        let o = ones[i];
        if z > o {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }
    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let strlen = lines[0].len();
    let (zeroes, ones) = get_bitcount(lines);
    let mut oxygen: Vec<String> = lines.to_vec();
    let mut co2: Vec<String> = lines.to_vec();
    for i in 0..strlen {
        oxygen = filter_input(oxygen, i, true);
        co2 = filter_input(co2, i, false);
    }
    let oxygen_parsed = i64::from_str_radix(&oxygen[0], 2).unwrap();
    let co2_parsed = i64::from_str_radix(&co2[0], 2).unwrap();
    oxygen_parsed * co2_parsed
}

#[cfg(test)]
mod test {
    fn test_input() -> Vec<String> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 198;
        let actual = crate::solutions::day03::part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 230;
        let actual = crate::solutions::day03::part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
