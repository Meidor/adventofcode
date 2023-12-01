use color_eyre::eyre::Result;

use crate::helpers::InputHelpers;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const REVERSE_DIGITS: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

pub fn find_first_digit(line: &str, count_words: bool, is_reversed: bool) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            return chars[i].to_digit(10).unwrap();
        }
        if !count_words {
            i += 1;
            continue;
        }

        let substr = line[i..].to_string();
        let dg = if is_reversed { REVERSE_DIGITS } else { DIGITS };
        for (j, digit) in dg.iter().enumerate() {
            if substr.starts_with(digit) {
                return j as u32 + 1;
            }
        }
        i += 1;
    }
    unreachable!("invalid input line: {}", line);
}

pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_digit = find_first_digit(line, false, false);
            let second_digit = find_first_digit(&line.reverse_string(), false, true);
            first_digit * 10 + second_digit
        })
        .sum::<u32>()
        .to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_digit = find_first_digit(line, true, false);
            let second_digit = find_first_digit(&line.reverse_string(), true, true);
            first_digit * 10 + second_digit
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
    }

    fn test_input_2() -> &'static str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "142";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "281";
        let actual = part_two(&test_input_2())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
