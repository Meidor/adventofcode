use color_eyre::eyre::Result;
use glam::ivec2;

use crate::helpers::Grid;

#[derive(Debug)]
struct Trees {
    pub shape: [usize; 2],
    pub trees: Vec<u8>,
}

impl Trees {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let size = width * height;
        let trees: Vec<u8> = input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        Self {
            shape: [width, height],
            trees,
        }
    }
}

impl Grid<u8> for Trees {
    fn width(&self) -> usize {
        self.shape[0]
    }

    fn height(&self) -> usize {
        self.shape[1]
    }

    fn values(&self) -> &[u8] {
        &self.trees
    }
}

pub fn part_one(input: &str) -> Result<String> {
    let trees = Trees::new(input);
    let mut visible = (trees.width() * 2) + ((trees.height() - 2) * 2);
    for y in 1..trees.height() - 1 {
        for x in 1..trees.width() - 1 {
            let pos = ivec2(x as i32, y as i32);
            let height = *trees.get_position(pos);
            let row = trees.get_row(y);
            let column = trees.get_column(x);
            if row[0..x].into_iter().all(|h| *h < height) {
                visible += 1;
                continue;
            }
            if row[x + 1..trees.width()].into_iter().all(|h| *h < height) {
                visible += 1;
                continue;
            }

            if column[0..y].into_iter().all(|h| *h < height) {
                visible += 1;
                continue;
            }

            if column[y + 1..trees.height()]
                .into_iter()
                .all(|h| *h < height)
            {
                visible += 1;
                continue;
            }
        }
    }
    Ok(visible.to_string())
}

fn get_score(target_height: u8, it: &mut dyn Iterator<Item = &u8>) -> usize {
    let mut score: usize = 0;
    loop {
        match it.next() {
            None => break,
            Some(nt) => {
                score += 1;
                if *nt >= target_height {
                    break;
                }
            }
        }
    }
    score
}

pub fn part_two(input: &str) -> Result<String> {
    let trees = Trees::new(input);
    let mut max_score: usize = 0;
    for y in 1..trees.height() - 1 {
        for x in 1..trees.width() - 1 {
            let pos = ivec2(x as i32, y as i32);
            let height = *trees.get_position(pos);
            let row = trees.get_row(y);
            let column = trees.get_column(x);
            let left_score = get_score(height, &mut row[0..x].iter().rev());
            let right_score = get_score(height, &mut row[x + 1..trees.width()].iter());          
            let up_score = get_score(height, &mut column[0..y].iter().rev());
            let down_score = get_score(height, &mut column[y + 1..trees.height()].iter());
            let score = left_score * right_score * up_score * down_score;
            if score > max_score {
                max_score = score;
            }
        }
    }
    Ok(max_score.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "30373
25512
65332
33549
35390
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "21";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "8";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
