use glam::{i64vec2, I64Vec2};

pub fn remove_collinear_points(polygon: &[I64Vec2]) -> Vec<I64Vec2> {
    let mut result: Vec<I64Vec2> = polygon
        .windows(3)
        .filter(|window| !is_collinear(window[0], window[1], window[2]))
        .map(|window| window[1])
        .collect();

    // Handle the first and last points
    if !is_collinear(
        polygon[polygon.len() - 2],
        polygon[polygon.len() - 1],
        polygon[0],
    ) {
        result.push(polygon[polygon.len() - 1]);
    }

    if !is_collinear(polygon[polygon.len() - 1], polygon[0], polygon[1]) {
        result.push(polygon[0]);
    }

    result
}

pub fn is_collinear(p0: I64Vec2, p1: I64Vec2, p2: I64Vec2) -> bool {
    (p1.y - p0.y) * (p2.x - p1.x) == (p2.y - p1.y) * (p1.x - p0.x)
}

pub fn point_in_polygon(point: I64Vec2, polygon: &[I64Vec2]) -> bool {
    winding_number(point, polygon) != 0
}

pub fn winding_number(point: I64Vec2, polygon: &[I64Vec2]) -> i32 {
    let mut winding_number = 0;

    // Loop through all edges of the polygon
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        if p1.y <= point.y {
            if p2.y > point.y && is_left(p1, p2, point) > 0 {
                winding_number += 1; // Upward crossing
            }
        } else if p2.y <= point.y && is_left(p1, p2, point) < 0 {
            winding_number -= 1; // Downward crossing
        }
    }

    winding_number
}

fn is_left(p1: I64Vec2, p2: I64Vec2, point: I64Vec2) -> i64 {
    ((p2.x - p1.x) * (point.y - p1.y) - (point.x - p1.x) * (p2.y - p1.y)).signum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindingOrder {
    Clockwise,
    CounterClockwise,
}

pub fn get_winding_order(path: &[I64Vec2]) -> Option<WindingOrder> {
    let mut sum = 0;
    for i in 0..path.len() {
        let p1 = path[i];
        let p2 = path[(i + 1) % path.len()];
        sum += (p2.x - p1.x) * (p2.y + p1.y);
    }
    match sum {
        x if x > 0 => Some(WindingOrder::Clockwise),
        x if x < 0 => Some(WindingOrder::CounterClockwise),
        _ => None,
    }
}

#[inline]
pub fn manhattan_distance(a: I64Vec2, b: I64Vec2) -> usize {
    let md = (a - b).abs();
    (md.x + md.y) as usize
}

pub fn count_line_points(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    num_integer::gcd((x1 - x0).abs(), (y1 - y0).abs()) + 1
}

pub fn count_polygon_border_points(polygon: &[I64Vec2]) -> i64 {
    let mut total_count = 0;
    if polygon.len() < 2 {
        return 0;
    }

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = if i + 1 < polygon.len() {
            polygon[i + 1]
        } else {
            polygon[0]
        };
        total_count += count_line_points(p1.x, p1.y, p2.x, p2.y);
    }

    // Subtract the number of vertices to account for the overlapping points
    total_count - polygon.len() as i64
}

// Shoelace formula
pub fn polygon_area(polygon: &[I64Vec2]) -> f64 {
    let mut area = 0.0;
    let n = polygon.len();

    for i in 0..n {
        let next_index = (i + 1) % n;
        area += (polygon[i].x as f64) * (polygon[next_index].y as f64);
        area -= (polygon[i].y as f64) * (polygon[next_index].x as f64);
    }

    area.abs() / 2.0
}

// Pick's theorem
pub fn points_in_polygon(polygon: &[I64Vec2]) -> usize {
    let a = polygon_area(polygon);
    let b = count_polygon_border_points(polygon) as f64;
    let i = a - b / 2.0 + 1.0;
    i as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection4 {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline]
    pub fn as_vec(self) -> I64Vec2 {
        match self {
            Self::Up => i64vec2(0, -1),
            Self::Down => i64vec2(0, 1),
            Self::Left => i64vec2(-1, 0),
            Self::Right => i64vec2(1, 0),
        }
    }

    #[inline]
    pub fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::i64vec2;
    use rstest::rstest;

    #[rstest]
    #[case(vec![
        i64vec2(0,0),
        i64vec2(0,1),
        i64vec2(1,1),
        i64vec2(1,0),
    ], 1.0f64)]
    pub fn polygon_area_test(#[case] polygon: Vec<I64Vec2>, #[case] expected: f64) {
        let actual = polygon_area(&polygon);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(&vec![
        i64vec2(0,0),
        i64vec2(0,4),
        i64vec2(4,4),
        i64vec2(4,0),
    ], 16)]
    #[case(&vec![i64vec2(0,0), i64vec2(5, 0), i64vec2(5, 2), i64vec2(3, 2), i64vec2(3, 4), i64vec2(0, 4)], 18)]
    #[case(&vec![i64vec2(6,0), i64vec2(6,5), i64vec2(4, 5), i64vec2(4, 7), i64vec2(6,7), i64vec2(6,9), i64vec2(1, 9), i64vec2(1, 7), i64vec2(0, 7), i64vec2(0, 5), i64vec2(2, 5), i64vec2(2, 2), i64vec2(0, 2), i64vec2(0, 0)], 38)]
    pub fn count_polygon_border_points_test(#[case] polygon: &[I64Vec2], #[case] expected: i64) {
        let actual = count_polygon_border_points(polygon);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec![
        i64vec2(0,0),
        i64vec2(0,1),
        i64vec2(1,1),
        i64vec2(1,0),
    ], 0)]
    #[case(vec![
        i64vec2(0,0),
        i64vec2(0,1),
        i64vec2(0,2),
        i64vec2(1,2),
        i64vec2(2,2),
        i64vec2(2,1),
        i64vec2(2,0),
        i64vec2(1,0),
    ], 1)]
    pub fn points_in_polygon_test(#[case] boundary: Vec<I64Vec2>, #[case] expected: usize) {
        let actual = points_in_polygon(&boundary);
        assert_eq!(actual, expected);
    }
}
