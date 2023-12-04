use std::ops::RangeInclusive;

use glam::{ivec2, IVec2};
use regex::Regex;

#[derive(Clone)]
struct TargetArea {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl TargetArea {
    pub fn from_input(lines: &[String]) -> Self {
        let re = Regex::new(r"^target area: x=(-?[0-9]+)\.\.(-?[0-9]+), y=(-?[0-9]+)..(-?[0-9]+)$")
            .unwrap();
        let captures = re.captures(&lines[0]).unwrap();
        let min_x: i32 = captures[1].parse().unwrap();
        let max_x: i32 = captures[2].parse().unwrap();
        let min_y: i32 = captures[3].parse().unwrap();
        let max_y: i32 = captures[4].parse().unwrap();
        Self {
            x_range: min_x..=max_x,
            y_range: min_y..=max_y,
        }
    }
}

struct Probe {
    velocity: IVec2,
    position: IVec2,
    max_height: i32,
    target_area: TargetArea,
}

impl Probe {
    pub fn new(velocity: IVec2, target_area: TargetArea) -> Self {
        Self {
            velocity,
            target_area,
            max_height: 0,
            position: ivec2(0, 0),
        }
    }

    pub fn is_in_target_area(&self) -> bool {
        self.target_area.x_range.contains(&self.position.x)
            && self.target_area.y_range.contains(&self.position.y)
    }

    pub fn step(&mut self) {
        self.position += self.velocity;
        self.max_height = self.position.y.max(self.max_height);
        self.velocity.x += match self.velocity.x {
            x if x < 0 => 1,
            x if x > 0 => -1,
            0 => 0,
            _ => unreachable!(),
        };
        self.velocity.y -= 1;
    }
}

fn run(probe: &mut Probe) {
    loop {
        probe.step();
        if probe.is_in_target_area()
            || probe.position.x > *probe.target_area.x_range.end()
            || probe.position.y < *probe.target_area.y_range.start()
        {
            break;
        }
    }
}

fn max_height(probe: &mut Probe) -> i32 {
    run(probe);
    if probe.is_in_target_area() {
        probe.max_height
    } else {
        i32::MIN
    }
}

fn reach_target(probe: &mut Probe) -> bool {
    run(probe);
    probe.is_in_target_area()
}

fn velocities(target_area: &TargetArea) -> Vec<IVec2> {
    let mut velocities: Vec<IVec2> = vec![];
    for y in *target_area.y_range.start()..=target_area.y_range.start().abs() {
        for x in -target_area.x_range.end()..=*target_area.x_range.end() {
            velocities.push(ivec2(x, y));
        }
    }
    velocities
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let target_area = TargetArea::from_input(lines);
    let velocities = velocities(&target_area);
    let max = velocities
        .into_iter()
        .map(|v| max_height(&mut Probe::new(v, target_area.clone())))
        .max()
        .unwrap();
    max.to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let target_area = TargetArea::from_input(lines);
    let velocities = velocities(&target_area);
    velocities
        .into_iter()
        .map(|v| reach_target(&mut Probe::new(v, target_area.clone())))
        .filter(|v| *v)
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec!["target area: x=20..30, y=-10..-5".to_string()]
    }

    #[test]
    fn test_part_one() {
        let expected = "45";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "112";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
