use color_eyre::eyre::Result;
use glam::{i64vec2, I64Vec2};
use helpers::{count_polygon_border_points, points_in_polygon, Direction};
use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(U|D|R|L) ([0-9]*) \(#([0-9a-z]*)\)$").unwrap());

trait DigInstruction {
    fn direction(&self) -> Direction;
    fn distance(&self) -> usize;
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
    true_instruction: String,
}

impl FromStr for Instruction {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RE.captures(s).unwrap();
        let direction = direction_from_char(captures[1].parse::<char>().unwrap());
        let distance = captures[2].parse::<usize>().unwrap();
        let true_instruction = captures[3].parse::<String>().unwrap();
        Ok(Instruction {
            direction,
            distance,
            true_instruction,
        })
    }
}

fn direction_from_char(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => unreachable!("Invalid direction char: {}", c),
    }
}

fn make_true(mut instruction: Instruction) -> Instruction {
    let true_instruction = &instruction.true_instruction;
    let distance = usize::from_str_radix(&true_instruction[0..5], 16).unwrap();
    let direction = match &true_instruction[5..] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => unreachable!("Invalid direction: {}", &true_instruction[6..=6]),
    };
    instruction.distance = distance;
    instruction.direction = direction;
    instruction
}

fn count_dug_holes(instructions: impl Iterator<Item = Instruction>) -> usize {
    let polygon = instructions.fold(vec![], |mut polygon: Vec<I64Vec2>, instruction| {
        let last = *polygon.last().unwrap_or(&i64vec2(0, 0));
        let direction = instruction.direction.as_vec() * instruction.distance as i64;
        polygon.push(last + i64vec2(direction.x, direction.y));
        polygon
    });

    let border = count_polygon_border_points(&polygon) as usize;
    let inner = points_in_polygon(&polygon);
    border + inner
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap());
    let result = count_dug_holes(instructions);
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let instructions = input
        .trim()
        .lines()
        .map(|l| make_true(l.parse::<Instruction>().unwrap()));
    let result = count_dug_holes(instructions);
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "62";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "952408144115";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
