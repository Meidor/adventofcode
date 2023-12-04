use std::collections::HashSet;

use color_eyre::eyre::Result;
use glam::{ivec2, IVec2};
use rayon::prelude::*;
use regex::Regex;

use helpers::distance;

#[derive(Debug, Clone)]
struct Sensor {
    pub pos: IVec2,
    pub closest_beacon: IVec2,
    pub distance_closest_beacon: usize,
}

impl Sensor {
    pub fn new(pos: IVec2, closest_beacon: IVec2) -> Self {
        Self {
            pos,
            closest_beacon,
            distance_closest_beacon: distance(pos, closest_beacon),
        }
    }

    pub fn is_in_coverage(&self, pos: IVec2) -> bool {
        distance(self.pos, pos) <= self.distance_closest_beacon
    }
}

fn get_sensors(input: &str) -> Result<Vec<Sensor>> {
    let re = Regex::new(
        r"^Sensor at x=(-?(?:\d)+), y=(-?(?:\d)+): closest beacon is at x=(-?(?:\d)+), y=(-?(?:\d)+)$",
    )?;
    Ok(input
        .trim()
        .lines()
        .map(|l| {
            let captures = re.captures(l).expect("invalid");
            let sx = captures[1].parse::<i32>().expect("invalid input");
            let sy = captures[2].parse::<i32>().expect("invalid input");
            let bx = captures[3].parse::<i32>().expect("invalid input");
            let by = captures[4].parse::<i32>().expect("invalid input");
            Sensor::new(ivec2(sx, sy), ivec2(bx, by))
        })
        .collect())
}

//TODO: DOE SLIMMER MAKEN
pub fn part_one_ext(input: &str, target_y: i32) -> Result<String> {
    let sensors = get_sensors(input)?;
    let sensor_loc = sensors.iter().map(|s| s.pos).collect::<HashSet<IVec2>>();
    let beacons = sensors
        .par_iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<IVec2>>();
    let xs = sensors
        .par_iter()
        .flat_map(|s| [s.pos.x, s.closest_beacon.x])
        .collect::<Vec<i32>>();
    let max_dist = sensors
        .par_iter()
        .map(|s| s.distance_closest_beacon)
        .max()
        .expect("no dist") as i32;
    let min_x = *xs.par_iter().min().expect("no values") - max_dist;
    let max_x = *xs.par_iter().max().expect("no values") + max_dist;
    let result = (min_x..max_x)
        .into_par_iter()
        .map(|x| {
            let pos = ivec2(x, target_y);
            if beacons.contains(&pos) || sensor_loc.contains(&pos) {
                return 0;
            }
            if sensors.iter().any(|s| s.is_in_coverage(pos)) {
                return 1;
            }
            0
        })
        .sum::<usize>();
    Ok(result.to_string())
}

pub fn part_one(input: &str) -> Result<String> {
    part_one_ext(input, 2000000)
}

//TODO: slimmer/sneller maken
pub fn part_two_ext(_input: &str, _upper_bound: usize) -> Result<String> {
    Ok(0.to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    part_two_ext(input, 4000000)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "26";
        let actual = part_one_ext(test_input(), 10)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = ((14 * 20) + 11).to_string();
        let actual = part_two_ext(test_input(), 20)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
