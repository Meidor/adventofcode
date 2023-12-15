use color_eyre::eyre::Result;
use helpers::Grid;
use std::str::FromStr;

#[derive(Debug)]
struct Pattern {
    pattern: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid<char> for Pattern {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn values(&self) -> &[char] {
        &self.pattern
    }

    fn values_mut(&mut self) -> &mut [char] {
        &mut self.pattern
    }
}

impl FromStr for Pattern {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let pattern = s
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<char>>();

        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        Ok(Self {
            pattern,
            width,
            height,
        })
    }
}

impl Pattern {
    fn find_reflection(&self) -> (Option<usize>, Option<usize>) {
        let mut transposed_pattern = self.pattern.clone();
        transpose::transpose(
            &self.pattern,
            &mut transposed_pattern,
            self.width,
            self.height,
        );

        let row_reflection = Self::find_reflection_row(&self.pattern, self.width, self.height);
        let column_reflection =
            Self::find_reflection_row(&transposed_pattern, self.height, self.width);
        (row_reflection, column_reflection)
    }

    fn check_reflection(i: usize, pattern: &[char], width: usize, height: usize) -> Option<usize> {
        if i == 0 {
            panic!("i cannot be 0");
        }

        let possible_result = i;
        let mut r1 = i - 1;
        let mut r2 = i;
        while r2 < height {
            let row_1 = &pattern[r1 * width..r1 * width + width];
            let row_2 = &pattern[r2 * width..r2 * width + width];
            if row_1 != row_2 {
                return None;
            }
            if r1 == 0 {
                break;
            }
            r1 -= 1;
            r2 += 1;
        }
        Some(possible_result)
    }

    fn find_reflection_row(pattern: &[char], width: usize, height: usize) -> Option<usize> {
        for i in 1..height {
            if let Some(result) = Self::check_reflection(i, pattern, width, height) {
                return Some(result);
            }
        }
        None
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let result = input
        .split("\n\n")
        .map(|s| s.parse::<Pattern>().unwrap().find_reflection())
        .fold(0, |acc, (row, column)| {
            acc + (column.unwrap_or(0) + (100 * row.unwrap_or(0)))
        });
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    Ok("0".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn test_input() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "405";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "400";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[rstest]
    #[case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.", (None, Some(5)))]
    #[case("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", (Some(4), None))]
    #[case("##..####..##..#
#.##.#..##..##.
##..###.##..##.
.#..#.#........
######.########
##..##...####..
##..###.#....#.
.###..#.######.
#.##.#..##..##.
#....#..##..##.
#....##.##..##.
######.##.##.##
.####..###..###", (None, Some(11)))]
    fn test_part_one_case(#[case] input: &str, #[case] expected: (Option<usize>, Option<usize>)) {
        let actual = input.parse::<Pattern>().unwrap().find_reflection();
        assert_eq!(expected, actual);
    }
}
