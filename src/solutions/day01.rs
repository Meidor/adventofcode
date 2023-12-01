use color_eyre::eyre::Result;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[tracing::instrument]
fn find_first_digit(line: &str, use_digits: bool, search_reverse: bool) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let length = chars.len() as i32;
    let mut i: i32 = if search_reverse { length - 1 } else { 0 };

    let increment: i32 = if search_reverse { -1 } else { 1 };
    while (0..length).contains(&i) {
        let ui = i as usize;
        if chars[ui].is_ascii_digit() {
            return chars[ui].to_digit(10).unwrap();
        }
        if !use_digits {
            i += increment;
            continue;
        }

        let substr = if search_reverse {
            &line[..=ui]
        } else {
            &line[ui..]
        };

        for (j, digit) in DIGITS.iter().enumerate() {
            if (search_reverse && substr.ends_with(digit))
                || (!search_reverse && substr.starts_with(digit))
            {
                return j as u32 + 1;
            }
        }
        i += increment;
    }
    unreachable!("invalid input line: {}", line);
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_digit = find_first_digit(line, false, false);
            let second_digit = find_first_digit(line, false, true);
            first_digit * 10 + second_digit
        })
        .sum::<u32>()
        .to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first_digit = find_first_digit(line, true, false);
            let second_digit = find_first_digit(line, true, true);
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
