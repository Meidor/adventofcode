use color_eyre::eyre::Result;
use glam::{i64vec2, I64Vec2};
use helpers::{manhattan_distance, Grid};
use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline]
    fn as_vec(self) -> I64Vec2 {
        match self {
            Self::Up => i64vec2(0, -1),
            Self::Down => i64vec2(0, 1),
            Self::Left => i64vec2(-1, 0),
            Self::Right => i64vec2(1, 0),
        }
    }

    #[inline]
    fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

struct CityGrid {
    blocks: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for CityGrid {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let blocks = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        Ok(Self {
            blocks,
            width,
            height,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    priority: usize,
    cost: usize,
    position: usize,
    direction: Direction,
    steps_direction: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for CityGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g: &dyn Grid<u8> = self;
        g.fmt(f)
    }
}

impl Grid<u8> for CityGrid {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[u8] {
        &self.blocks
    }

    fn values_mut(&mut self) -> &mut [u8] {
        &mut self.blocks
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct CostKey {
    position: usize,
    direction: Direction,
    steps_direction: usize,
}

impl CityGrid {
    fn find_lowest_cost(
        &mut self,
        start: I64Vec2,
        end: I64Vec2,
        min_straight: usize,
        max_straight: usize,
    ) -> Option<usize> {
        let start_index = self.get_index(start);
        let end_index = self.get_index(end);
        let mut frontier = BinaryHeap::new();
        let mut costs: HashMap<CostKey, usize> = HashMap::new();

        let directions = &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        costs.insert(
            CostKey {
                position: start_index,
                direction: Direction::Right,
                steps_direction: 0,
            },
            0,
        );
        costs.insert(
            CostKey {
                position: start_index,
                direction: Direction::Down,
                steps_direction: 0,
            },
            0,
        );
        frontier.push(State {
            cost: 0,
            priority: 0,
            position: start_index,
            direction: Direction::Right,
            steps_direction: 0,
        });

        let manhattan_distances: Vec<usize> = self
            .blocks
            .iter()
            .enumerate()
            .map(|(i, _)| manhattan_distance(self.position_from_index(i), end))
            .collect();

        while let Some(State {
            cost,
            priority: _,
            position,
            direction,
            steps_direction,
        }) = frontier.pop()
        {
            let pos = self.position_from_index(position);
            if position == end_index
                && steps_direction >= min_straight
                && steps_direction <= max_straight
            {
                return Some(cost);
            }

            let dist_key = CostKey {
                position,
                direction,
                steps_direction,
            };

            if let Some(&existing_cost) = costs.get(&dist_key) {
                if cost > existing_cost {
                    continue;
                }
            }

            for dir in directions.iter().filter(|&d| *d != direction.invert()) {
                let new_position = pos + dir.as_vec();
                if !self.has_position(new_position) {
                    continue;
                }
                let new_index = self.get_index(new_position);
                let next = State {
                    position: new_index,
                    direction: *dir,
                    steps_direction: if *dir == direction {
                        steps_direction + 1
                    } else {
                        1
                    },
                    cost: cost + self.get_cost(new_position),
                    priority: cost + self.get_cost(new_position) + manhattan_distances[new_index],
                };

                if (direction == *dir || steps_direction >= min_straight)
                    && next.steps_direction <= max_straight
                {
                    let cost_key = CostKey {
                        position: new_index,
                        direction: *dir,
                        steps_direction: next.steps_direction,
                    };
                    match costs.entry(cost_key) {
                        Entry::Occupied(o) if next.priority < *o.get() => {
                            *o.into_mut() = next.priority;
                            frontier.push(next);
                        }
                        Entry::Vacant(v) => {
                            v.insert(next.priority);
                            frontier.push(next);
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }

    #[inline]
    fn get_cost(&self, position: I64Vec2) -> usize {
        let pos = self.get_position(position);
        *pos as usize
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut cg: CityGrid = input.parse()?;
    let start = i64vec2(0, 0);
    let end = i64vec2(cg.width() as i64 - 1, cg.height() as i64 - 1);
    let lowest_cost = cg.find_lowest_cost(start, end, 0, 3).unwrap();
    Ok(lowest_cost.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut cg: CityGrid = input.parse()?;
    let start = i64vec2(0, 0);
    let end = i64vec2(cg.width() as i64 - 1, cg.height() as i64 - 1);
    let lowest_cost = cg.find_lowest_cost(start, end, 4, 10).unwrap();
    Ok(lowest_cost.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "102";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "94";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
