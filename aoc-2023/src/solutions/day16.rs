use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};
use helpers::Grid;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty(bool),
    HorizontalSplitter(bool),
    VerticalSplitter(bool),
    LeftAngledMirror(bool),
    RightAngledMirror(bool),
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Empty(false),
            '|' => Tile::VerticalSplitter(false),
            '-' => Tile::HorizontalSplitter(false),
            '/' => Tile::RightAngledMirror(false),
            '\\' => Tile::LeftAngledMirror(false),
            _ => unreachable!("invalid input"),
        }
    }

    fn is_energized(&self) -> bool {
        match self {
            Tile::Empty(e) => *e,
            Tile::HorizontalSplitter(e) => *e,
            Tile::VerticalSplitter(e) => *e,
            Tile::LeftAngledMirror(e) => *e,
            Tile::RightAngledMirror(e) => *e,
        }
    }

    fn energize(&mut self) {
        match self {
            Tile::Empty(e) => *e = true,
            Tile::HorizontalSplitter(e) => *e = true,
            Tile::VerticalSplitter(e) => *e = true,
            Tile::LeftAngledMirror(e) => *e = true,
            Tile::RightAngledMirror(e) => *e = true,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty(e) => {
                if *e {
                    '#'
                } else {
                    '.'
                }
            }
            Tile::HorizontalSplitter(_) => '-',
            Tile::VerticalSplitter(_) => '|',
            Tile::RightAngledMirror(_) => '/',
            Tile::LeftAngledMirror(_) => '\\',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone)]
struct Contraption {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let g: &dyn Grid<Tile> = self;
        g.fmt(f)
    }
}

impl FromStr for Contraption {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles = s
            .trim()
            .replace('\n', "")
            .chars()
            .map(Tile::from_char)
            .collect();
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        Ok(Self {
            tiles,
            width,
            height,
        })
    }
}

impl Grid<Tile> for Contraption {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[Tile] {
        &self.tiles
    }

    fn values_mut(&mut self) -> &mut [Tile] {
        &mut self.tiles
    }
}

impl Contraption {
    fn trace_light(
        &mut self,
        position: IVec2,
        direction: IVec2,
        cache: &mut HashSet<(IVec2, IVec2)>,
    ) {
        if !self.has_position(position) || !cache.insert((position, direction)) {
            return;
        }

        let tile: &mut Tile = self.get_position_mut(position);
        tile.energize();

        match tile {
            Tile::Empty(_) => self.trace_light(position + direction, direction, cache),
            Tile::HorizontalSplitter(_) => {
                if direction.y == 0 {
                    self.trace_light(position + direction, direction, cache);
                } else {
                    self.trace_light(position + ivec2(-1, 0), ivec2(-1, 0), cache);
                    self.trace_light(position + ivec2(1, 0), IVec2::new(1, 0), cache);
                }
            }
            Tile::VerticalSplitter(_) => {
                if direction.x == 0 {
                    self.trace_light(position + direction, direction, cache);
                } else {
                    self.trace_light(position + ivec2(0, -1), ivec2(0, -1), cache);
                    self.trace_light(position + ivec2(0, 1), IVec2::new(0, 1), cache);
                }
            }
            Tile::LeftAngledMirror(_) => {
                if direction.y == 0 && direction.x > 0 {
                    self.trace_light(position + ivec2(0, 1), ivec2(0, 1), cache)
                } else if direction.y == 0 && direction.x < 0 {
                    self.trace_light(position + ivec2(0, -1), ivec2(0, -1), cache)
                } else if direction.x == 0 && direction.y > 0 {
                    self.trace_light(position + ivec2(1, 0), ivec2(1, 0), cache)
                } else if direction.x == 0 && direction.y < 0 {
                    self.trace_light(position + ivec2(-1, 0), ivec2(-1, 0), cache)
                }
            }
            Tile::RightAngledMirror(_) => {
                if direction.y == 0 && direction.x > 0 {
                    self.trace_light(position + ivec2(0, -1), ivec2(0, -1), cache)
                } else if direction.y == 0 && direction.x < 0 {
                    self.trace_light(position + ivec2(0, 1), ivec2(0, 1), cache)
                } else if direction.x == 0 && direction.y > 0 {
                    self.trace_light(position + ivec2(-1, 0), ivec2(-1, 0), cache)
                } else if direction.x == 0 && direction.y < 0 {
                    self.trace_light(position + ivec2(1, 0), ivec2(1, 0), cache)
                }
            }
        }
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut contraption: Contraption = input.parse()?;
    contraption.trace_light(ivec2(0, 0), ivec2(1, 0), &mut HashSet::new());
    let result = contraption
        .tiles
        .iter()
        .filter(|t| t.is_energized())
        .count();
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let contraption: Contraption = input.parse()?;
    let width = contraption.width() as i32;
    let height = contraption.height() as i32;
    let top_row = (0..width).map(move |x| (ivec2(x, 0), ivec2(0, 1)));
    let bottom_row = (0..width).map(move |x| (ivec2(x, height - 1), ivec2(0, -1)));
    let left_column = (1..height - 1).map(move |y| (ivec2(0, y), ivec2(1, 0)));
    let right_column = (1..height - 1).map(move |y| (ivec2(width - 1, y), ivec2(-1, 0)));
    let start_positions = top_row
        .chain(bottom_row)
        .chain(left_column)
        .chain(right_column)
        .collect_vec();

    let result = start_positions
        .par_iter()
        .map(|&start| {
            let mut contraption = contraption.clone();
            contraption.trace_light(start.0, start.1, &mut HashSet::new());
            contraption
                .tiles
                .iter()
                .filter(|t| t.is_energized())
                .count()
        })
        .max()
        .unwrap();
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "46";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "51";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
