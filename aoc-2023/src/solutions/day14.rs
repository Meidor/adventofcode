use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use color_eyre::eyre::Result;
use glam::ivec2;
use helpers::Grid;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum DishContent {
    Round,
    Cube,
    None,
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

impl ParabollicDish {
    pub fn calculate_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pos = ivec2(x as i32, y as i32);
                let content = self.get_position(pos);
                if content == &DishContent::Round {
                    load += self.height - y;
                }
            }
        }
        load
    }

    fn find_cycle(&mut self) -> (usize, usize) {
        let mut cache: HashMap<Vec<DishContent>, usize> = HashMap::new();
        cache.insert(self.grid.clone(), 0);
        let mut cycle = 0;
        loop {
            self.cycle();
            cycle += 1;
            if let Some(c) = cache.insert(self.grid.clone(), cycle) {
                return (c, cycle);
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
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

    fn tilt_north(&mut self) {
        let mut did_swap = true;
        while did_swap {
            did_swap = false;
            for y in 1..self.height() {
                for x in 0..self.width() {
                    let pos = ivec2(x as i32, y as i32);
                    let pos_up = ivec2(x as i32, y as i32 - 1);
                    let up = self.get_position(pos_up);
                    let current = self.get_position(pos);
                    if current == &DishContent::Round && up == &DishContent::None {
                        self.set_position(pos_up, DishContent::Round);
                        self.set_position(pos, DishContent::None);
                        did_swap = true;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut did_swap = true;
        while did_swap {
            did_swap = false;
            for y in (0..self.height() - 1).rev() {
                for x in 0..self.width() {
                    let pos = ivec2(x as i32, y as i32);
                    let pos_down = ivec2(x as i32, y as i32 + 1);
                    let down = self.get_position(pos_down);
                    let current = self.get_position(pos);
                    if current == &DishContent::Round && down == &DishContent::None {
                        self.set_position(pos_down, DishContent::Round);
                        self.set_position(pos, DishContent::None);
                        did_swap = true;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut did_swap = true;
        while did_swap {
            did_swap = false;
            for y in 0..self.height() {
                for x in (0..self.width() - 1).rev() {
                    let pos = ivec2(x as i32, y as i32);
                    let pos_right = ivec2(x as i32 + 1, y as i32);
                    let right = self.get_position(pos_right);
                    let current = self.get_position(pos);
                    if current == &DishContent::Round && right == &DishContent::None {
                        self.set_position(pos_right, DishContent::Round);
                        self.set_position(pos, DishContent::None);
                        did_swap = true;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut did_swap = true;
        while did_swap {
            did_swap = false;
            for y in 0..self.height() {
                for x in 1..self.width() {
                    let pos = ivec2(x as i32, y as i32);
                    let pos_left = ivec2(x as i32 - 1, y as i32);
                    let left = self.get_position(pos_left);
                    let current = self.get_position(pos);
                    if current == &DishContent::Round && left == &DishContent::None {
                        self.set_position(pos_left, DishContent::Round);
                        self.set_position(pos, DishContent::None);
                        did_swap = true;
                    }
                }
            }
        }
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut dish = input.parse::<ParabollicDish>()?;
    dish.tilt_north();
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
    use rstest::rstest;

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

    #[rstest]
    #[case(
        test_input(),
        1,
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
    )]
    #[case(
        test_input(),
        2,
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
    )]
    #[case(
        test_input(),
        3,
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
    )]
    fn test_cycling(#[case] input: &str, #[case] cycle_count: usize, #[case] expected: &str) {
        let mut dish = input.parse::<ParabollicDish>().unwrap();
        for _ in 0..cycle_count {
            dish.cycle();
        }
        let actual = dish.to_string();
        println!("EXPECTED:");
        println!("{}", expected);
        println!("ACTUAL:");
        println!("{}", actual);
    }
}
