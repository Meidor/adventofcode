use color_eyre::eyre::Result;

fn winning_amount(time: usize, record: usize) -> usize {
    let t = time as f64;
    let y = record as f64;

    let root1 = t / 2.0 - (t.powi(2) - 4.0 * y).sqrt() / 2.0;
    let root2 = t / 2.0 + (t.powi(2) - 4.0 * y).sqrt() / 2.0;
    let lower_bound = root1.ceil() as usize;
    let upper_bound = root2.floor() as usize;
    let mut count = upper_bound - lower_bound + 1;
    if root1.fract() == 0.0 {
        count -= 1;
    }
    if root2.fract() == 0.0 {
        count -= 1;
    }
    count
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let numbers = input
        .replace("Time:", "")
        .replace("Distance:", "")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let (times, records) = numbers.split_at(numbers.len() / 2);
    let result = times
        .iter()
        .zip(records)
        .map(|(t, r)| winning_amount(*t, *r))
        .product::<usize>();
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let numbers = input
        .replace("Time:", "")
        .replace("Distance:", "")
        .replace(' ', "")
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let time = numbers[0];
    let record = numbers[1];
    Ok(winning_amount(time, record).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "288";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "71503";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
