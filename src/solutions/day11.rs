use crate::helpers::Grid;
use glam::{ivec2, IVec2};

struct Octopuses {
    width: usize,
    height: usize,
    values: Vec<u8>,
    flashes: usize,
    step: usize,
}

impl Grid<u8> for Octopuses {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[u8] {
        &self.values
    }
}

impl Octopuses {
    pub fn new(values: Vec<u8>) -> Self {
        Self {
            width: 10,
            height: 10,
            values,
            flashes: 0,
            step: 0,
        }
    }

    pub fn print(&self) {
        println!("after step {}", self.step);
        println!();
        for r in 0..self.height {
            let start = r * self.width;
            let end = r * self.width + self.width;
            let values = &self.values[start..end];
            println!("{:?}", values);
        }
        println!();
    }

    pub fn step(&mut self, days: usize) {
        for d in 0..days {
            for i in 0..self.values.len() {
                self.values[i] += 1;
            }

            let mut has_flashed = true;
            let mut flash_pos: Vec<IVec2> = vec![];
            while has_flashed {
                has_flashed = false;
                for y in 0..self.width {
                    for x in 0..self.height {
                        let pos = ivec2(x as i32, y as i32);
                        if *self.get_position(pos) > 9 {
                            has_flashed = true;
                            flash_pos.push(pos);
                            self.flashes += 1;
                            let i = self.get_index(pos);
                            self.values[i] = 0;
                            let nbs = self.get_neighbours(pos, true);
                            for nb in nbs {
                                let i = self.get_index(nb);
                                self.values[i] += 1;
                            }
                        }
                    }
                }
            }
            for p in flash_pos {
                let i = self.get_index(p);
                self.values[i] = 0;
            }
            self.step += 1;
        }
    }
}

fn parse_input(lines: &[String]) -> Vec<u8> {
    lines
        .iter()
        .flat_map(|l| l.chars())
        .map(|c| format!("{}", c).parse().unwrap())
        .collect()
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let mut octopuses = Octopuses::new(parse_input(lines));
    octopuses.step(100);
    octopuses.flashes.to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let mut octopuses = Octopuses::new(parse_input(lines));
    while octopuses.values.iter().any(|f| *f != 0) {
        octopuses.step(1);
    }
    octopuses.step.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "1656";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "195";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
