use color_eyre::eyre::Result;

use crate::helpers;

fn get_answer(input: &str, window_size: usize) -> Result<usize> {
    let chars: Vec<char> = input
        .lines()
        .nth(0)
        .expect("Invalid input")
        .chars()
        .collect();
    let mut i: usize = window_size;
    for window in chars.windows(window_size) {
        if helpers::has_unique_elements(window) {
            return Ok(i);
        }
        i += 1;
    }
    unreachable!();
}

pub fn part_one(input: &str) -> Result<String> {
    Ok(get_answer(input, 4)?.to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    Ok(get_answer(input, 14)?.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<(&'static str, &'static str)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7"),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "6"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"),
        ]
    }

    #[test]
    fn test_part_one() -> Result<()> {
        for (input, expected) in test_input() {
            let actual = part_one(input)?;
            assert_eq!(expected, actual);
        }
        Ok(())
    }

    fn test_input2() -> Vec<(&'static str, &'static str)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "19"),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "23"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"),
        ]
    }

    #[test]
    fn test_part_two() -> Result<()> {
        for (input, expected) in test_input2() {
            let actual = part_two(input)?;
            assert_eq!(expected, actual);
        }
        Ok(())
    }
}
