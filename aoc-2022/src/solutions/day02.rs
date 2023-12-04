use regex::Regex;
use color_eyre::eyre::Result;
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl MatchResult {
    fn parse(input: &str) -> MatchResult {
        match input {
            "X" => MatchResult::Lose,
            "Y" => MatchResult::Draw,
            "Z" => MatchResult::Win,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    pub fn get_move(&self, result: MatchResult) -> Shape {
        match (self, result) {
            (Shape::Rock, MatchResult::Win) => Shape::Paper,
            (Shape::Rock, MatchResult::Lose) => Shape::Scissors,
            (Shape::Rock, MatchResult::Draw) => Shape::Rock,
            (Shape::Paper, MatchResult::Win) => Shape::Scissors,
            (Shape::Paper, MatchResult::Lose) => Shape::Rock,
            (Shape::Paper, MatchResult::Draw) => Shape::Paper,
            (Shape::Scissors, MatchResult::Win) => Shape::Rock,
            (Shape::Scissors, MatchResult::Lose) => Shape::Paper,
            (Shape::Scissors, MatchResult::Draw) => Shape::Scissors,
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


pub fn part_one(input: &str) -> Result<String> {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    Ok(input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let you = Shape::parse(&captures[2]);
            let opponent = Shape::parse(&captures[1]);
            let shape_points = you.get_points();
            let match_points = you.match_up(opponent);
            
            shape_points + match_points
        })
        .sum::<i32>()
        .to_string())
}


pub fn part_two(input: &str) -> Result<String> {
    let re = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    Ok(input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let captures = re.captures(l).unwrap();
            let result = MatchResult::parse(&captures[2]);
            let opponent = Shape::parse(&captures[1]);
            let you = opponent.get_move(result);
            let shape_points = you.get_points();
            let match_points = you.match_up(opponent);
            
            shape_points + match_points
        })
        .sum::<i32>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "A Y
B X
C Z
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "15";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "12";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
