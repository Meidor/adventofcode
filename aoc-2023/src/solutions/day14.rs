use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    hash::DefaultHasher,
    hash::{Hash, Hasher},
    rc::Rc,
    str::FromStr,
};

use color_eyre::eyre::Result;
use glam::{i64vec2, I64Vec2};
use helpers::{FilterGrid, Grid};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum DishContent {
    Round,
    Cube,
    None,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for DishContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DishContent::Round => write!(f, "O"),
            DishContent::Cube => write!(f, "#"),
            DishContent::None => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct ParabollicDish {
    grid: Vec<DishContent>,
    width: usize,
    height: usize,
}

impl Display for ParabollicDish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let g: &dyn Grid<DishContent> = self;
        g.fmt(f)
    }
}

impl FromStr for ParabollicDish {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = s
            .trim()
            .replace('\n', "")
            .chars()
            .map(|c| match c {
                'O' => DishContent::Round,
                '#' => DishContent::Cube,
                '.' => DishContent::None,
                _ => unreachable!("Invalid character {}", c),
            })
            .collect::<Vec<DishContent>>();
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

impl Grid<DishContent> for ParabollicDish {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[DishContent] {
        &self.grid
    }

    fn values_mut(&mut self) -> &mut [DishContent] {
        &mut self.grid
    }
}

impl FilterGrid<DishContent> for ParabollicDish {}

type SortKeyFn = Rc<dyn Fn(&I64Vec2) -> i64>;

impl ParabollicDish {
    pub fn calculate_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = i64vec2(x as i64, y as i64);
                let content = self.get_position(pos);
                if content == &DishContent::Round {
                    load += self.height - y;
                }
            }
        }
        load
    }

    fn find_cycle(&mut self) -> (usize, usize) {
        let mut cache: HashMap<u64, usize> = HashMap::new();
        let mut hasher = DefaultHasher::new();
        self.grid.hash(&mut hasher);
        let hash = hasher.finish();
        cache.insert(hash, 0);
        let mut cycle = 0;
        loop {
            self.cycle();
            cycle += 1;
            hasher = DefaultHasher::new();
            self.grid.hash(&mut hasher);
            let hash = hasher.finish();
            if let Some(c) = cache.insert(hash, cycle) {
                return (c, cycle);
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    pub fn tilt_cycle(&mut self, cycles: usize) {
        let cycle = self.find_cycle();
        let cycle_length = cycle.1 - cycle.0;
        if cycles < cycle.0 {
            for _ in 0..cycles {
                self.cycle();
            }
        } else {
            let remaining_cycles = (cycles - cycle.0) % cycle_length;
            for _ in 0..remaining_cycles {
                self.cycle();
            }
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let mut positions = self.filter_positions(|d| d == &DishContent::Round);
        let (sort_key_fn, offset): (SortKeyFn, I64Vec2) = match direction {
            Direction::North => (Rc::new(|a| a.y), i64vec2(0, -1)),
            Direction::South => (Rc::new(|a| -a.y), i64vec2(0, 1)),
            Direction::East => (Rc::new(|a| -a.x), i64vec2(1, 0)),
            Direction::West => (Rc::new(|a| a.x), i64vec2(-1, 0)),
        };
        // Sort positions
        positions.sort_by_key(|k| (sort_key_fn)(k));
        let mut did_swap = true;
        while did_swap {
            did_swap = false;
            let mut new_positions = Vec::<I64Vec2>::with_capacity(positions.len());
            for p in &positions {
                let new_pos = *p + offset;
                if self.has_position(new_pos) && self.get_position(new_pos) == &DishContent::None {
                    self.set_position(new_pos, DishContent::Round);
                    self.set_position(*p, DishContent::None);
                    new_positions.push(new_pos);
                    did_swap = true;
                }
            }
            // Sort new_positions
            new_positions.sort_by_key(|k| (sort_key_fn)(k));
            positions = new_positions;
        }
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut dish = input.parse::<ParabollicDish>()?;
    dish.tilt(Direction::North);
    Ok(dish.calculate_load().to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut dish = input.parse::<ParabollicDish>()?;
    dish.tilt_cycle(1_000_000_000);
    Ok(dish.calculate_load().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "136";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "64";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
