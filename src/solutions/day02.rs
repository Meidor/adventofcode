use std::str::FromStr;

use crate::helpers::InputHelpers;
use color_eyre::eyre::Result;

#[derive(Debug)]
struct Game {
    id: usize,
    moves: Vec<Move>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.moves.iter().all(|m| m.is_valid())
    }

    pub fn min_power(&self) -> u64 {
        let (max_red, max_green, max_blue) = self.moves.iter().flat_map(|m| &m.cubes).fold(
            (0, 0, 0),
            |(max_red, max_green, max_blue), cube| match cube {
                Cube::Red(rc) => (max_red.max(*rc), max_green, max_blue),
                Cube::Green(gc) => (max_red, max_green.max(*gc), max_blue),
                Cube::Blue(bc) => (max_red, max_green, max_blue.max(*bc)),
            },
        );
        (max_red * max_green * max_blue) as u64
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let colon = s.find(':').expect("missing colon");
        let id = s[5..colon].parse::<usize>().expect("invalid id");
        Ok(Game {
            id,
            moves: s[colon + 2..]
                .split(';')
                .map(|m| m.trim().parse::<Move>().expect("invalid move"))
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Move {
    cubes: Vec<Cube>,
}

impl Move {
    pub fn is_valid(&self) -> bool {
        self.cubes.iter().all(|cube| cube.is_valid())
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let cubes = s
            .split(',')
            .map(|cube| {
                let cube = cube.trim();
                let (count, color) = cube.split_at(cube.find(' ').expect("missing space"));
                let count = count.parse::<usize>().expect("invalid count");
                let color = color.trim();
                match color {
                    "red" => Cube::Red(count),
                    "green" => Cube::Green(count),
                    "blue" => Cube::Blue(count),
                    _ => panic!("invalid color"),
                }
            })
            .collect();
        Ok(Move { cubes })
    }
}

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Cube {
    pub fn is_valid(&self) -> bool {
        match self {
            Cube::Red(rc) => *rc <= 12,
            Cube::Green(gc) => *gc <= 13,
            Cube::Blue(bc) => *bc <= 14,
        }
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .parse_input::<Game>()
        .expect("invalid game")
        .iter()
        .filter_map(|game| if game.is_valid() { Some(game.id) } else { None })
        .sum::<usize>()
        .to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    Ok(input
        .parse_input::<Game>()
        .expect("invalid game")
        .iter()
        .map(|game| game.min_power())
        .sum::<u64>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "8";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "2286";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
