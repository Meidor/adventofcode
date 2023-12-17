use glam::IVec2;

pub fn remove_collinear_points(polygon: &[IVec2]) -> Vec<IVec2> {
    let mut result: Vec<IVec2> = polygon
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

pub fn is_collinear(p0: IVec2, p1: IVec2, p2: IVec2) -> bool {
    (p1.y - p0.y) * (p2.x - p1.x) == (p2.y - p1.y) * (p1.x - p0.x)
}

pub fn point_in_polygon(point: IVec2, polygon: &[IVec2]) -> bool {
    winding_number(point, polygon) != 0
}

pub fn winding_number(point: IVec2, polygon: &[IVec2]) -> i32 {
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

fn is_left(p1: IVec2, p2: IVec2, point: IVec2) -> i32 {
    ((p2.x - p1.x) * (point.y - p1.y) - (point.x - p1.x) * (p2.y - p1.y)).signum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindingOrder {
    Clockwise,
    CounterClockwise,
}

pub fn get_winding_order(path: &[IVec2]) -> Option<WindingOrder> {
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
pub fn manhattan_distance(a: IVec2, b: IVec2) -> usize {
    let md = (a - b).abs();
    (md.x + md.y) as usize
}

// Shoelace formula
pub fn polygon_area(polygon: &[IVec2]) -> f64 {
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
pub fn points_in_polygon(polygon: &[IVec2]) -> usize {
    let a = polygon_area(polygon);
    let b = polygon.len() as f64;
    let i = a - b / 2.0 + 1.0;
    i as usize
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::ivec2;
    use rstest::rstest;

    #[rstest]
    #[case(vec![
        ivec2(0,0),
        ivec2(0,1),
        ivec2(1,1),
        ivec2(1,0),
    ], 1.0f64)]
    pub fn polygon_area_test(#[case] polygon: Vec<IVec2>, #[case] expected: f64) {
        let actual = polygon_area(&polygon);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec![
        ivec2(0,0),
        ivec2(0,1),
        ivec2(1,1),
        ivec2(1,0),
    ], 0)]
    #[case(vec![
        ivec2(0,0),
        ivec2(0,1),
        ivec2(0,2),
        ivec2(1,2),
        ivec2(2,2),
        ivec2(2,1),
        ivec2(2,0),
        ivec2(1,0),
    ], 1)]
    pub fn points_in_polygon_test(#[case] boundary: Vec<IVec2>, #[case] expected: usize) {
        let actual = points_in_polygon(&boundary);
        assert_eq!(actual, expected);
    }
}
