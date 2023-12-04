use std::collections::HashSet;

use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};

fn rope_sim(input: &str, length: usize) -> usize {
    let mut rope = vec![ivec2(0, 0); length];
    input
        .lines()
        .filter(|l| *l != "\n")
        .flat_map(|l| {
            let split: Vec<&str> = l.split(' ').collect();
            let instruction = split[0];
            let count = split[1].parse::<usize>().expect("invalid input");
            let mut result: Vec<&str> = vec![];
            for _i in 0..count {
                result.push(instruction);
            }
            result
        })
        .fold(HashSet::<IVec2>::new(), |mut acc, instruction| {
            rope[0] += match instruction {
                "U" => ivec2(0, 1),
                "D" => ivec2(0, -1),
                "L" => ivec2(-1, 0),
                "R" => ivec2(1, 0),
                _ => unreachable!("Invalid instruction in input"),
            };
            for x in 0..rope.len() - 1 {
                let dist = rope[x] - rope[x + 1];
                if dist.x.abs() > 1 || dist.y.abs() > 1 {
                    rope[x + 1] += ivec2(dist.x.clamp(-1, 1), dist.y.clamp(-1, 1));
                }
            }
            acc.insert(rope[rope.len() - 1]);
            acc
        })
        .len()
}

pub fn part_one(input: &str) -> Result<String> {
    Ok(rope_sim(input, 2).to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    Ok(rope_sim(input, 10).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "13";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "1";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
