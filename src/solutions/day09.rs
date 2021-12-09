use std::collections::HashSet;

use glam::{ivec2, IVec2};

use crate::helpers::Grid;

struct HeightMap {
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl Grid<u8> for HeightMap {
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

impl HeightMap {
    fn is_lowpoint(&self, pos: IVec2) -> bool {
        !self
            .get_neighbours(pos, false)
            .into_iter()
            .any(|n| self.get_position(pos) >= self.get_position(n))
    }

    pub fn get_lowpoints(&self) -> Vec<IVec2> {
        let mut result: Vec<IVec2> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = ivec2(x as i32, y as i32);
                if self.is_lowpoint(pos) {
                    result.push(pos);
                }
            }
        }
        result
    }

    pub fn get_basin(&self, pos: IVec2, acc: &mut HashSet<IVec2>) {
        let height = *self.get_position(pos);
        let candidates: Vec<IVec2> = self
            .get_neighbours(pos, false)
            .into_iter()
            .filter(|n| {
                let sh = *self.get_position(*n);
                sh > height && sh < 9
            })
            .collect();
        for candidate in candidates {
            acc.insert(candidate);
            self.get_basin(candidate, acc);
        }
    }
}

fn parse_input(lines: &[String]) -> HeightMap {
    let height = lines.len();
    let width = lines[0].chars().count();
    let values: Vec<u8> = lines
        .iter()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect();
    HeightMap {
        width,
        height,
        values,
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let map = parse_input(lines);
    map.get_lowpoints()
        .into_iter()
        .map(|p| *map.get_position(p) as i64 + 1)
        .sum()
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let map = parse_input(lines);
    let mut sizes: Vec<usize> = map
        .get_lowpoints()
        .into_iter()
        .map(|p| {
            let mut basin: HashSet<IVec2> = HashSet::new();
            basin.insert(p);
            map.get_basin(p, &mut basin);
            basin.len()
        })
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    (sizes.into_iter().take(3).reduce(|a, b| a * b).unwrap()) as i64
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 15;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 1134;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
