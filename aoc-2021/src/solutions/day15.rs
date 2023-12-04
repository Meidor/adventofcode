use std::collections::HashMap;

use glam::{ivec2, IVec2};
use priority_queue::DoublePriorityQueue;

use crate::helpers::Grid;

struct Cave {
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl Grid<u8> for Cave {
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

impl Cave {
    pub fn from_input(lines: &[String]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let values: Vec<u8> = lines
            .iter()
            .flat_map(|l| l.chars())
            .map(|c| format!("{}", c).parse().unwrap())
            .collect();
        Self {
            width,
            height,
            values,
        }
    }

    pub fn from_input_expanded(lines: &[String]) -> Self {
        let base_height = lines.len();
        let base_width = lines[0].len();
        let width = base_width * 5;
        let height = base_height * 5;

        let mut values: Vec<u8> = vec![255; width * height];
        for (yy, line) in lines.iter().enumerate() {
            let vs: Vec<u8> = line
                .chars()
                .map(|c| format!("{}", c).parse().unwrap())
                .collect();
            for (xx, v) in vs.iter().enumerate() {
                for y in 0..5 {
                    let row = yy + base_height * y;
                    for x in 0..5 {
                        let column = xx + base_width * x;
                        let i = column + row * width;

                        let mut risk = *v + x as u8 + y as u8;
                        if risk > 9 {
                            risk %= 9;
                            if risk == 0 {
                                risk = 9;
                            }
                        }
                        values[i] = risk;
                    }
                }
            }
        }

        Self {
            width,
            height,
            values,
        }
    }

    fn cost(&self, node: IVec2) -> i32 {
        *self.get_position(node) as i32
    }

    fn h(&self, start: IVec2, goal: IVec2) -> i32 {
        (start.x - goal.x).abs() + (start.y - goal.y).abs()
    }

    fn get_total_risk(
        &self,
        start: IVec2,
        goal: IVec2,
        came_from: HashMap<IVec2, Option<IVec2>>,
    ) -> i32 {
        let mut risk: i32 = 0;
        let mut current = goal;
        while current != start {
            risk += self.cost(current);
            current = came_from.get(&current).unwrap().unwrap();
        }
        risk
    }

    fn find_lowest_risk_path(&self, start: IVec2, goal: IVec2) -> i32 {
        let mut frontier = DoublePriorityQueue::<IVec2, i32>::new();
        let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::new();
        let mut cost_so_far: HashMap<IVec2, i32> = HashMap::new();
        came_from.insert(start, None);
        cost_so_far.insert(start, 0);

        frontier.push(start, 0);

        while !frontier.is_empty() {
            let current_val = frontier.pop_min().unwrap();
            let current = current_val.0;
            if current == goal {
                break;
            }
            for next in self.get_neighbours(current, false) {
                let new_cost = cost_so_far.get(&current).unwrap() + self.cost(next);
                let c = *cost_so_far.get(&next).unwrap_or(&i32::MAX);
                if new_cost < c {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost + self.h(goal, next);
                    frontier.push(next, priority);
                    came_from.insert(next, Some(current));
                }
            }
        }
        self.get_total_risk(start, goal, came_from)
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let cave = Cave::from_input(lines);
    let start: IVec2 = ivec2(0, 0);
    let goal: IVec2 = ivec2((cave.width - 1) as i32, (cave.height - 1) as i32);
    cave.find_lowest_risk_path(start, goal).to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let cave = Cave::from_input_expanded(lines);
    let start = ivec2(0, 0);
    let goal: IVec2 = ivec2((cave.width - 1) as i32, (cave.height - 1) as i32);
    cave.find_lowest_risk_path(start, goal).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "1163751742".to_string(),
            "1381373672".to_string(),
            "2136511328".to_string(),
            "3694931569".to_string(),
            "7463417111".to_string(),
            "1319128137".to_string(),
            "1359912421".to_string(),
            "3125421639".to_string(),
            "1293138521".to_string(),
            "2311944581".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "40";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "315";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
