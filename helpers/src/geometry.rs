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

pub fn distance(a: IVec2, b: IVec2) -> usize {
    let md = (a - b).abs();
    (md.x + md.y) as usize
}