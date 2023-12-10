use std::{collections::HashSet, fmt::Display, str::FromStr};

use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};
use helpers::{point_in_polygon, FilterGrid, Grid};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Animal,
    Ground,
    Inside,
    Outside,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::NorthSouth => '┃',
            Pipe::WestEast => '━',
            Pipe::NorthEast => '┗',
            Pipe::NorthWest => '┛',
            Pipe::SouthWest => '┓',
            Pipe::SouthEast => '┏',
            Pipe::Animal => 'S',
            Pipe::Ground => '▓',
            Pipe::Inside => ' ',
            Pipe::Outside => '▓',
        };
        write!(f, "{}", c)
    }
}

impl Pipe {
    pub fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::NorthSouth,
            '-' => Pipe::WestEast,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Animal,
            'I' => Pipe::Inside,
            'O' => Pipe::Outside,
            _ => unreachable!("invalid input pipe"),
        }
    }
}

#[derive(Debug)]
struct PipeMaze {
    pipes: Vec<Pipe>,
    width: usize,
    height: usize,
    animal: Option<IVec2>,
}

impl PipeMaze {
    fn find_and_replace_animal(&mut self) {
        let animal_index = self.pipes.iter().position(|&p| p == Pipe::Animal).unwrap();
        let animal_pos = ivec2(
            (animal_index % self.width) as i32,
            (animal_index / self.width) as i32,
        );
        self.animal = Some(animal_pos);
        let neighborhood = self.get_neighborhood(animal_pos);

        let directions = [
            (
                neighborhood.north,
                [Pipe::NorthSouth, Pipe::SouthEast, Pipe::SouthWest],
            ),
            (
                neighborhood.east,
                [Pipe::WestEast, Pipe::NorthWest, Pipe::SouthWest],
            ),
            (
                neighborhood.south,
                [Pipe::NorthSouth, Pipe::NorthEast, Pipe::NorthWest],
            ),
            (
                neighborhood.west,
                [Pipe::WestEast, Pipe::NorthEast, Pipe::SouthEast],
            ),
        ];

        let mut connections = [false; 4];
        for (i, (direction, pipes)) in directions.iter().enumerate() {
            if let Some(np) = direction {
                connections[i] = pipes.contains(self.get_position(*np));
            }
        }

        let pipe = match connections {
            [true, false, true, false] => Pipe::NorthSouth,
            [false, true, false, true] => Pipe::WestEast,
            [true, false, false, true] => Pipe::NorthWest,
            [true, true, false, false] => Pipe::NorthEast,
            [false, true, true, false] => Pipe::SouthEast,
            [false, false, true, true] => Pipe::SouthWest,
            _ => unreachable!("invalid animal connections"),
        };

        self.set_position(animal_pos, pipe);
    }

    fn find_first_cycle(&self) -> Option<Vec<IVec2>> {
        let start_node = self.animal.expect("animal should have been found");
        let mut visited: HashSet<IVec2> = HashSet::new();
        let mut path: Vec<IVec2> = vec![];
        let mut cycle = vec![];
        if self.dfs_cycle_detection(start_node, start_node, &mut visited, &mut path, &mut cycle) {
            Some(cycle)
        } else {
            None
        }
    }

    fn dfs_cycle_detection(
        &self,
        node: IVec2,
        start_node: IVec2,
        visited: &mut HashSet<IVec2>,
        path: &mut Vec<IVec2>,
        cycle: &mut Vec<IVec2>,
    ) -> bool {
        visited.insert(node);
        path.push(node);
        let current_pipe = self.get_position(node);
        let mut next_nodes: Vec<IVec2> = match current_pipe {
            Pipe::NorthSouth => vec![node + ivec2(0, -1), node + ivec2(0, 1)],
            Pipe::WestEast => vec![node + ivec2(-1, 0), node + ivec2(1, 0)],
            Pipe::NorthEast => vec![node + ivec2(0, -1), node + ivec2(1, 0)],
            Pipe::NorthWest => vec![node + ivec2(0, -1), node + ivec2(-1, 0)],
            Pipe::SouthWest => vec![node + ivec2(0, 1), node + ivec2(-1, 0)],
            Pipe::SouthEast => vec![node + ivec2(0, 1), node + ivec2(1, 0)],
            Pipe::Animal => unreachable!("should have been replaced by real pipe"),
            Pipe::Ground => vec![],
            Pipe::Inside => vec![],
            Pipe::Outside => vec![],
        };
        next_nodes.retain(|f| self.has_position(*f));

        for next_node in next_nodes {
            if next_node == start_node && path.len() > 2 {
                // Found a cycle; add it to the cycle vector and return true
                cycle.extend_from_slice(path);
                return true;
            } else if !visited.contains(&next_node)
                && self.dfs_cycle_detection(next_node, start_node, visited, path, cycle)
            {
                return true;
            }
        }
        path.pop();
        visited.retain(|&n| n != node);
        false
    }

    fn count_contained(&mut self, path: &[IVec2]) -> usize {
        let (min_x, max_x, min_y, max_y) = path.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(min_x, max_x, min_y, max_y), p| {
                (
                    min_x.min(p.x),
                    max_x.max(p.x),
                    min_y.min(p.y),
                    max_y.max(p.y),
                )
            },
        );

        let path_set: HashSet<_> = path.iter().collect();

        (0..self.pipes.len())
            .filter_map(|i| {
                let pos = ivec2((i % self.width) as i32, (i / self.width) as i32);
                if pos.x >= min_x
                    && pos.x <= max_x
                    && pos.y >= min_y
                    && pos.y <= max_y
                    && !path_set.contains(&pos)
                    && point_in_polygon(pos, path)
                {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }
}

impl FromStr for PipeMaze {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let input = s.trim();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let pipes = input
            .lines()
            .flat_map(|line| line.chars().map(Pipe::from_char))
            .collect();

        Ok(Self {
            pipes,
            width,
            height,
            animal: None,
        })
    }
}

impl Grid<Pipe> for PipeMaze {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[Pipe] {
        &self.pipes
    }

    fn values_mut(&mut self) -> &mut [Pipe] {
        &mut self.pipes
    }
}

impl FilterGrid<Pipe> for PipeMaze {}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let mut maze: PipeMaze = input.parse()?;
    maze.find_and_replace_animal();
    let cycle = maze.find_first_cycle().unwrap();
    let max = cycle.len() / 2;
    Ok(max.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut maze: PipeMaze = input.parse()?;
    maze.find_and_replace_animal();
    let cycle = maze.find_first_cycle().unwrap();
    Ok(maze.count_contained(&cycle).to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_input_simple() -> &'static str {
        ".....
.S-7.
.|.|.
.L-J.
.....
"
    }

    fn test_input_simple_noise() -> &'static str {
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"
    }

    fn test_input_complex() -> &'static str {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
    }

    fn test_input_complex_noise() -> &'static str {
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"
    }

    fn test_input_part_two_simple() -> &'static str {
        "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
    }

    fn test_input_part_two_complex() -> &'static str {
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"
    }

    #[rstest]
    #[case(test_input_simple(), 4)]
    #[case(test_input_simple_noise(), 4)]
    #[case(test_input_complex(), 8)]
    #[case(test_input_complex_noise(), 8)]
    fn test_part_one(#[case] input: &'static str, #[case] expected: usize) -> Result<()> {
        let actual = part_one(input)?;
        assert_eq!(expected.to_string(), actual);
        Ok(())
    }

    #[rstest]
    #[case(test_input_simple(), 1)]
    #[case(test_input_part_two_simple(), 4)]
    #[case(test_input_part_two_complex(), 10)]
    fn test_part_two(#[case] input: &'static str, #[case] expected: usize) -> Result<()> {
        let actual = part_two(input)?;
        assert_eq!(expected.to_string(), actual);
        Ok(())
    }
}
