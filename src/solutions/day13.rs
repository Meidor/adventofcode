use glam::{ivec2, IVec2};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}
struct Paper {
    dots: HashSet<IVec2>,
    instructions: Vec<Fold>,
}

impl Paper {
    fn from_input(lines: &[String]) -> Self {
        let parts: Vec<&[String]> = lines.split(|l| l.is_empty()).collect();
        let dots: HashSet<IVec2> = parts[0]
            .iter()
            .map(|i| {
                let c: Vec<&str> = i.split(',').collect();
                ivec2(c[0].parse().unwrap(), c[1].parse().unwrap())
            })
            .collect();
        let re = Regex::new(r"^fold along (x|y)=([0-9]+)$").unwrap();
        let instructions: Vec<Fold> = parts[1]
            .iter()
            .map(|l| {
                let captures = re.captures(l).unwrap();
                let t = &captures[1];
                let amount = &captures[2];
                match t {
                    "x" => Fold::Vertical(amount.parse().unwrap()),
                    "y" => Fold::Horizontal(amount.parse().unwrap()),
                    _ => panic!("shouldn't be here"),
                }
            })
            .collect();
        Self { dots, instructions }
    }

    fn fold_once(&mut self, fold: Fold) {
        match fold {
            Fold::Horizontal(y) => {
                let to_move: Vec<IVec2> = self.dots.drain_filter(|d| d.y >= y).collect();
                to_move.iter().for_each(|d| {
                    let new_y = y - (d.y - y);
                    self.dots.insert(ivec2(d.x, y - (d.y - y)));
                });
            }
            Fold::Vertical(x) => {
                let to_move: Vec<IVec2> = self.dots.drain_filter(|d| d.x >= x).collect();
                to_move.iter().for_each(|d| {
                    self.dots.insert(ivec2(x - (d.x - x), d.y));
                });
            }
        }
    }

    fn fold(&mut self, times: usize) {
        for i in 0..times {
            self.fold_once(self.instructions[i]);
        }
    }

    fn count_dots(&self) -> usize {
        self.dots.len()
    }

    fn print(&self) {
        let max_x = self.dots.iter().map(|d| d.x).max().unwrap();
        let max_y = self.dots.iter().map(|d| d.y).max().unwrap();
        for y in 0..=max_y {
            let mut line: String = String::new();
            for x in 0..=max_x {
                let coord = ivec2(x, y);
                if self.dots.contains(&coord) {
                    line += "#";
                } else {
                    line += ".";
                }
            }
            println!("{}", line);
        }
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let mut paper = Paper::from_input(lines);
    paper.fold(1);
    paper.count_dots() as i64
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let mut paper = Paper::from_input(lines);
    paper.fold(paper.instructions.len());
    println!("part two:");
    println!("```");
    paper.print();
    println!("```");
    paper.count_dots() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 17;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 16;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
