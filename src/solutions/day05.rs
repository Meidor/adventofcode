use std::collections::HashMap;

use glam::{uvec2, UVec2};

struct LineSegment(UVec2, UVec2);

impl LineSegment {
    pub fn is_straight(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    pub fn get_coordinates(&self) -> Vec<UVec2> {
        let mut result = vec![];
        let a = self.0;
        let b = self.1;

        let mut curr_x = a.x;
        let mut curr_y = a.y;

        loop {
            if curr_x == b.x && curr_y == b.y {
                result.push(b);
                break;
            }
            result.push(uvec2(curr_x, curr_y));

            match curr_x.cmp(&b.x) {
                std::cmp::Ordering::Less => curr_x += 1,
                std::cmp::Ordering::Greater => curr_x -= 1,
                std::cmp::Ordering::Equal => (),
            }

            match curr_y.cmp(&b.y) {
                std::cmp::Ordering::Less => curr_y += 1,
                std::cmp::Ordering::Greater => curr_y -= 1,
                std::cmp::Ordering::Equal => (),
            }
        }
        result
    }
}

struct VentSystem {
    vents: HashMap<UVec2, usize>,
}

impl VentSystem {
    pub fn new() -> Self {
        Self {
            vents: HashMap::new(),
        }
    }

    pub fn add_segment(&mut self, segment: &LineSegment) {
        for coord in segment.get_coordinates() {
            *self.vents.entry(coord).or_insert(0) += 1;
        }
    }
}

fn parse_coord(str: &str) -> UVec2 {
    let c: Vec<u32> = str.split(',').map(|c| c.parse().unwrap()).collect();
    uvec2(c[0], c[1])
}

fn parse_input(lines: &[String]) -> Vec<LineSegment> {
    let mut segments: Vec<LineSegment> = Vec::with_capacity(lines.len());
    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let start = parse_coord(parts[0]);
        let end = parse_coord(parts[1]);
        segments.push(LineSegment(start, end));
    }
    segments
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let segments = parse_input(lines);
    let mut vent_system = VentSystem::new();
    for segment in segments.iter().filter(|s| s.is_straight()) {
        vent_system.add_segment(segment);
    }
    vent_system.vents.into_values().filter(|v| *v > 1).count().to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let segments = parse_input(lines);
    let mut vent_system = VentSystem::new();
    for segment in segments {
        vent_system.add_segment(&segment);
    }
    vent_system.vents.into_values().filter(|v| *v > 1).count().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "5";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "12";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }

    fn assert_coords(expected: Vec<UVec2>, actual: Vec<UVec2>) {
        assert_eq!(expected.len(), actual.len());
        for coord in actual {
            assert!(expected.contains(&coord));
        }
    }

    #[test]
    fn test_line_segment() {
        let segment = LineSegment(uvec2(9, 7), uvec2(7, 7));
        let actual = segment.get_coordinates();
        let expected = vec![uvec2(7, 7), uvec2(8, 7), uvec2(9, 7)];
        assert_coords(expected, actual);

        let segment = LineSegment(uvec2(1, 1), uvec2(3, 3));
        let actual = segment.get_coordinates();
        let expected = vec![uvec2(1, 1), uvec2(2, 2), uvec2(3, 3)];
        assert_coords(expected, actual);

        let segment = LineSegment(uvec2(9, 7), uvec2(7, 9));
        let actual = segment.get_coordinates();
        let expected = vec![uvec2(9, 7), uvec2(8, 8), uvec2(7, 9)];
        assert_coords(expected, actual);
    }
}
