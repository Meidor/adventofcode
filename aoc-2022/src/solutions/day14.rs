use std::{collections::HashSet, fmt::Display, iter};

use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};

struct Cave {
    sand_spawn_point: IVec2,
    rocks: HashSet<IVec2>,
    current_sand: Vec<IVec2>,
    settled_sand: HashSet<IVec2>,
    bottom: i32,
    has_floor: bool,
}

impl Cave {
    pub fn new(input: &str, sand_spawn_point: IVec2, has_floor: bool) -> Self {
        let rocks = Cave::get_rocks(input);
        let mut bottom = rocks.iter().map(|r| r.y).max().unwrap();
        if has_floor {
            bottom += 2;
        }
        Self {
            sand_spawn_point,
            rocks,
            bottom,
            has_floor,
            current_sand: vec![sand_spawn_point],
            settled_sand: HashSet::new(),
        }
    }

    fn is_free(&self, location: &IVec2) -> bool {
        !(self.settled_sand.contains(location)
            || self.rocks.contains(location)
            || (self.has_floor && location.y >= self.bottom))
    }

    fn get_next_location(&self, current_location: IVec2) -> Option<IVec2> {
        let n = current_location + ivec2(0, 1);
        if self.is_free(&n) {
            return Some(n);
        }

        let n = current_location + ivec2(-1, 1);
        if self.is_free(&n) {
            return Some(n);
        }

        let n = current_location + ivec2(1, 1);
        if self.is_free(&n) {
            return Some(n);
        }
        None
    }

    pub fn simulate_abyss(&mut self) -> usize {
        loop {
            if let Some(position) = self.tick() {
                if position.y > self.bottom {
                    break;
                }
            }
        }
        self.settled_sand.len()
    }

    pub fn simulate_floor(&mut self) -> usize {
        loop {
            self.tick();
            if self.settled_sand.contains(&self.sand_spawn_point) {
                break;
            }
        }
        self.settled_sand.len()
    }

    fn tick(&mut self) -> Option<IVec2> {
        let current = *self.current_sand.last().expect("stack shouldn't be empty");
        let next = self.get_next_location(current);
        if next.is_none() {
            self.settled_sand
                .insert(self.current_sand.pop().expect("stack shouldn't be empty"));
        } else {
            self.current_sand.push(next.unwrap());
        }
        next
    }

    fn get_rocks(input: &str) -> HashSet<IVec2> {
        input
            .trim()
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|c| {
                        let coord_parts: Vec<i32> =
                            c.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
                        ivec2(coord_parts[0], coord_parts[1])
                    })
                    .collect()
            })
            .flat_map(|rd: Vec<IVec2>| {
                let mut res: HashSet<IVec2> = HashSet::new();
                for i in 0..rd.len() - 1 {
                    let s = rd[i];
                    let e = rd[i + 1];
                    for y in i32::min(s.y, e.y)..=i32::max(s.y, e.y) {
                        for x in i32::min(s.x, e.x)..=i32::max(s.x, e.x) {
                            res.insert(ivec2(x, y));
                        }
                    }
                }
                res
            })
            .collect()
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rock_x = self.rocks.iter().map(|r| r.x);
        let sand_x = self.settled_sand.iter().map(|s| s.x);
        let spawn_x = iter::once(self.sand_spawn_point.x);
        let x_coords: Vec<i32> = rock_x.chain(sand_x).chain(spawn_x).collect();
        let padding = 0;
        let min_x = *x_coords.iter().min().unwrap() - padding;
        let max_x = *x_coords.iter().max().unwrap() + padding;

        let mut str = String::new();
        for y in 0..=self.bottom {
            for x in min_x..max_x {
                let pos = ivec2(x, y);
                if self.rocks.contains(&pos) || (self.has_floor && y >= self.bottom) {
                    str += "█";
                } else if self.settled_sand.contains(&pos) {
                    str += "●";
                } else if self.current_sand.contains(&pos) {
                    str += "◍";
                } else if pos == self.sand_spawn_point {
                    str += "⊕";
                } else {
                    str += "░";
                }
            }
            str += "\n";
        }
        write!(f, "{}", str)
    }
}

const SAND_SPAWN: IVec2 = IVec2 { x: 500, y: 0 };

pub fn part_one(input: &str) -> Result<String> {
    let mut cave = Cave::new(input, SAND_SPAWN, false);
    let result = cave.simulate_abyss();
    Ok(result.to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    let mut cave = Cave::new(input, SAND_SPAWN, true);
    let result = cave.simulate_floor();
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "24";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "93";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
