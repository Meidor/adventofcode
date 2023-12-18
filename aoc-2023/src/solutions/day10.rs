use color_eyre::eyre::Result;
use glam::{i64vec2, I64Vec2};
use helpers::{FilterGrid, Grid};
use std::{collections::HashSet, fmt::Display, str::FromStr};

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
    animal: Option<I64Vec2>,
}

impl PipeMaze {
    fn find_and_replace_animal(&mut self) {
        let animal_index = self.pipes.iter().position(|&p| p == Pipe::Animal).unwrap();
        let animal_pos = i64vec2(
            (animal_index % self.width) as i64,
            (animal_index / self.width) as i64,
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

    fn get_connections(&self, pos: I64Vec2) -> Vec<I64Vec2> {
        let pipe = *self.get_position(pos);
        let neighborhood = self.get_neighborhood(pos);

        let north = neighborhood.north;
        let east = neighborhood.east;
        let south = neighborhood.south;
        let west = neighborhood.west;

        let mut result = vec![];
        match pipe {
            Pipe::NorthSouth => {
                if let Some(n) = north {
                    result.push(n)
                }
                if let Some(s) = south {
                    result.push(s)
                }
            }
            Pipe::WestEast => {
                if let Some(w) = west {
                    result.push(w)
                }
                if let Some(e) = east {
                    result.push(e)
                }
            }
            Pipe::NorthEast => {
                if let Some(n) = north {
                    result.push(n)
                }
                if let Some(e) = east {
                    result.push(e)
                }
            }
            Pipe::NorthWest => {
                if let Some(n) = north {
                    result.push(n)
                }
                if let Some(w) = west {
                    result.push(w)
                }
            }
            Pipe::SouthWest => {
                if let Some(s) = south {
                    result.push(s)
                }
                if let Some(w) = west {
                    result.push(w)
                }
            }
            Pipe::SouthEast => {
                if let Some(s) = south {
                    result.push(s)
                }
                if let Some(e) = east {
                    result.push(e)
                }
            }
            Pipe::Animal => unreachable!("animal should have been replaced"),
            Pipe::Ground => unreachable!("ground should not be in the loop"),
            Pipe::Inside => unreachable!("inside should not be in the loop"),
            Pipe::Outside => unreachable!("outside should not be in the loop"),
        };
        result
    }

    fn find_cycle(&self) -> Vec<I64Vec2> {
        let mut cycle = vec![];
        let start = self.animal.expect("animal should have been found");
        let mut visited = HashSet::<I64Vec2>::new();
        visited.insert(start);
        cycle.push(start);
        let mut previous = i64vec2(-1, -1);
        let mut current = start;
        loop {
            let next = self
                .get_connections(current)
                .into_iter()
                .find(|f| *f != previous)
                .unwrap();
            previous = current;
            current = next;
            if !visited.insert(current) {
                break;
            } else {
                cycle.push(current);
            }
        }
        cycle
    }

    fn count_contained(&mut self, path: &[I64Vec2]) -> usize {
        helpers::points_in_polygon(path)
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
    let cycle = maze.find_cycle();
    let max = cycle.len() / 2;
    Ok(max.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let mut maze: PipeMaze = input.parse()?;
    maze.find_and_replace_animal();
    let cycle = maze.find_cycle();
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
