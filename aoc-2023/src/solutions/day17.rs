use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};
use helpers::{manhattan_distance, Grid};
use std::{
    collections::{BinaryHeap, HashMap},
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
    fn as_vec(self) -> IVec2 {
        match self {
            Self::Up => ivec2(0, -1),
            Self::Down => ivec2(0, 1),
            Self::Left => ivec2(-1, 0),
            Self::Right => ivec2(1, 0),
        }
    }

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
        start: IVec2,
        end: IVec2,
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
            if costs.contains_key(&dist_key) && cost > costs[&dist_key] {
                continue;
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
                    priority: cost
                        + self.get_cost(new_position)
                        + manhattan_distance(new_position, end),
                };

                let dist_key = CostKey {
                    position: new_index,
                    direction: *dir,
                    steps_direction: next.steps_direction,
                };

                if (direction == *dir || steps_direction >= min_straight)
                    && next.steps_direction <= max_straight
                    && (!costs.contains_key(&dist_key) || next.priority < costs[&dist_key])
                {
                    frontier.push(next);
                    costs.insert(dist_key, next.priority);
                }
            }
        }
        None
    }

    fn get_cost(&self, position: IVec2) -> usize {
        let pos = self.get_position(position);
        *pos as usize
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut cg: CityGrid = input.parse()?;
    let start = ivec2(0, 0);
    let end = ivec2(cg.width() as i32 - 1, cg.height() as i32 - 1);
    let lowest_cost = cg.find_lowest_cost(start, end, 0, 3).unwrap();
    Ok(lowest_cost.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut cg: CityGrid = input.parse()?;
    let start = ivec2(0, 0);
    let end = ivec2(cg.width() as i32 - 1, cg.height() as i32 - 1);
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
