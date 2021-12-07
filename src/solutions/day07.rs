fn parse_input(lines: &[String]) -> Vec<i64> {
    lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_fuel_costs(depth: i64, inputs: Vec<i64>, func: &dyn Fn(i64, i64) -> i64) -> Vec<i64> {
    inputs.into_iter().map(|i| func(i, depth)).collect()
}

fn solve(lines: &[String], fuel_cost: &dyn Fn(i64, i64) -> i64) -> i64 {
    let positions = parse_input(lines);
    let mut fuel_costs: Vec<i64> = vec![];
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    for depth in min..=max {
        let costs = get_fuel_costs(depth, positions.clone(), fuel_cost);
        fuel_costs.push(costs.into_iter().sum());
    }
    fuel_costs.into_iter().min().unwrap()
}

fn fuel_one(a: i64, b: i64) -> i64 {
    (a - b).abs()
}

fn fuel_two(a: i64, b: i64) -> i64 {
    let dist = (a - b).abs();
    dist * (dist + 1) / 2
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    solve(lines, &fuel_one)
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    solve(lines, &fuel_two)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec!["16,1,2,0,4,2,7,1,2,14".to_string()]
    }

    #[test]
    fn test_part_one() {
        let expected = 37;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 168;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
