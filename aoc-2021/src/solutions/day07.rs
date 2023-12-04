fn parse_input(lines: &[String]) -> Vec<i64> {
    lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve(positions: Vec<i64>, fuel_cost: &dyn Fn(i64, i64) -> i64) -> i64 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|d| positions.clone().into_iter().map(|i| fuel_cost(i, d)).sum())
        .min()
        .unwrap()
}

fn fuel_one(a: i64, b: i64) -> i64 {
    (a - b).abs()
}

fn fuel_two(a: i64, b: i64) -> i64 {
    let dist = (a - b).abs();
    dist * (dist + 1) / 2
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let positions = parse_input(lines);
    solve(positions, &fuel_one).to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let positions = parse_input(lines);
    solve(positions, &fuel_two).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec!["16,1,2,0,4,2,7,1,2,14".to_string()]
    }

    #[test]
    fn test_part_one() {
        let expected = "37";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "168";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
