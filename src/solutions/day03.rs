use itertools::Itertools;
use color_eyre::eyre::Result;


fn get_priority(char: char) -> u64 {
    if char.is_ascii_lowercase() {
        (char as u64) - 96
    } else {
        (char as u64) - 38
    }
}


pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|rucksack| {
            let items = rucksack.len();
            let left = &rucksack[0..items / 2];
            let right = &rucksack[items / 2..];
            let right_chars = right.chars();
            for char in left.chars() {
                if right.contains(char) {
                    return get_priority(char);
                }
            }
            unreachable!()
        })
        .sum::<u64>()
        .to_string())
}


pub fn part_two(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let backpacks: Vec<&str> = group.collect();
            let a = backpacks[0];
            let b = backpacks[1];
            let c = backpacks[2];
            for char in a.chars() {
                if b.contains(char) && c.contains(char) {
                    return get_priority(char);
                }
            }
            unreachable!();
        })
        .sum::<u64>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "157";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "70";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
