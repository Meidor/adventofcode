use std::{collections::HashSet, fmt::Display, str::FromStr};

use color_eyre::eyre::Result;
use glam::{i64vec2, I64Vec2};
use helpers::{manhattan_distance, Grid};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GalaxyMapItem {
    Empty,
    Galaxy,
}

impl Display for GalaxyMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GalaxyMapItem::Empty => '.',
            GalaxyMapItem::Galaxy => '#',
        };
        write!(f, "{}", c)
    }
}

impl GalaxyMapItem {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid map item"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GalaxyMap {
    map: Vec<GalaxyMapItem>,
    width: usize,
    height: usize,
}

impl GalaxyMap {
    fn find_galaxy_locations(&self, expansion_size: usize) -> HashSet<I64Vec2> {
        let mut expand_rows: Vec<usize> = vec![0; self.height];
        let mut expand_columns: Vec<usize> = vec![0; self.width];

        for y in 1..self.height {
            expand_rows[y] = expand_rows[y - 1]
                + if self.get_row(y - 1).iter().all(|&item| item == GalaxyMapItem::Empty) {
                    expansion_size
                } else {
                    0
                };
        }
        for x in 1..self.width {
            expand_columns[x] = expand_columns[x - 1]
                + if self.get_column(x - 1).iter().all(|&item| item == GalaxyMapItem::Empty) {
                    expansion_size
                } else {
                    0
                };
        }

        self.map
            .iter()
            .enumerate()
            .filter_map(|(pos, &item)| {
                if item == GalaxyMapItem::Galaxy {
                    let x = (pos % self.width) as i64;
                    let y = (pos / self.width) as i64;
                    let new_x = x + expand_columns[x as usize] as i64;
                    let new_y = y + expand_rows[y as usize] as i64;
                    Some(i64vec2(new_x, new_y))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_sum_shortest_paths(&self, expansion_size: usize) -> usize {
        let galaxies = self.find_galaxy_locations(expansion_size);
        let mut sum = 0;
        let galaxies_vec: Vec<_> = galaxies.iter().collect();
        for j in 0..galaxies_vec.len() {
            for i in j+1..galaxies_vec.len() {
                let a = *galaxies_vec[j];
                let b = *galaxies_vec[i];
                sum += manhattan_distance(a, b);
            }
        }
        sum
    }
}

impl Grid<GalaxyMapItem> for GalaxyMap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[GalaxyMapItem] {
        &self.map
    }

    fn values_mut(&mut self) -> &mut [GalaxyMapItem] {
        &mut self.map
    }
}

impl FromStr for GalaxyMap {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let map = s
            .lines()
            .flat_map(|line| line.chars().map(GalaxyMapItem::from_char))
            .collect::<Vec<_>>();
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        Ok(Self { map, width, height })
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let map: GalaxyMap = input.parse::<GalaxyMap>()?;
    let sum = map.find_sum_shortest_paths(1);
    Ok(sum.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let map: GalaxyMap = input.parse::<GalaxyMap>()?;
    let sum = map.find_sum_shortest_paths(999_999);
    Ok(sum.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"
    }

    #[rstest]
    #[case(test_input(), "374")]
    fn test_part_one(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        let actual = part_one(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[rstest]
    #[case(9, 1030)]
    #[case(99, 8410)]
    fn test_part_two(#[case] expansion_size: usize, #[case] expected: usize) -> Result<()> {
        let map: GalaxyMap = test_input().parse::<GalaxyMap>()?;
        let actual = map.find_sum_shortest_paths(expansion_size);
        assert_eq!(expected, actual);
        Ok(())
    }
}
