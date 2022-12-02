use regex::Regex;
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn parse(input: &str) -> Result {
        match input {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    pub fn get_move(&self, result: Result) -> Shape {
        match (self, result) {
            (Shape::Rock, Result::Win) => Shape::Paper,
            (Shape::Rock, Result::Lose) => Shape::Scissors,
            (Shape::Rock, Result::Draw) => Shape::Rock,
            (Shape::Paper, Result::Win) => Shape::Scissors,
            (Shape::Paper, Result::Lose) => Shape::Rock,
            (Shape::Paper, Result::Draw) => Shape::Paper,
            (Shape::Scissors, Result::Win) => Shape::Rock,
            (Shape::Scissors, Result::Lose) => Shape::Paper,
            (Shape::Scissors, Result::Draw) => Shape::Scissors,
        }
    }
    pub fn get_points(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    pub fn match_up(&self, other: Shape) -> i32 {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
        }
    }

    pub fn parse(input: &str) -> Shape {
        match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        }
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    lines
        .iter()
        .filter(|l| *l != "")
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let you = Shape::parse(&captures[2]);
            let opponent = Shape::parse(&captures[1]);
            let shape_points = you.get_points();
            let match_points = you.match_up(opponent);
            let score = shape_points + match_points;
            score
        })
        .sum::<i32>()
        .to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    lines
        .into_iter()
        .filter(|l| *l != "")
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let result = Result::parse(&captures[2]);
            let opponent = Shape::parse(&captures[1]);
            let you = opponent.get_move(result);
            let shape_points = you.get_points();
            let match_points = you.match_up(opponent);
            let score = shape_points + match_points;
            score
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
            "".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "15";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "12";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
