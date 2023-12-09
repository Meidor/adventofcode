use color_eyre::eyre::Result;

fn parse_numbers(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let numbers = parse_numbers(input);

    let mut result: i32 = 0;
    for line in numbers {
        let mut next = *line.iter().last().unwrap();
        let mut difference = Vec::<i32>::with_capacity(line.len() - 1);
        let mut current = line.clone();
        loop {
            current.iter().enumerate().for_each(|(i, n)| {
                if i != 0 {
                    let prev = current[i - 1];
                    difference.push(n - prev);
                }
            });
            next += *difference.last().unwrap();
            if difference.iter().all(|n| *n == 0) {
                break;
            }
            current = difference.clone();
            difference.clear();
        }
        result += next;
    }

    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let numbers = parse_numbers(input);

    let mut result: i32 = 0;
    for line in numbers {
        let mut steps: Vec<i32> = Vec::with_capacity(line.len() - 1);
        steps.push(line[0]);
        let mut difference = Vec::<i32>::with_capacity(line.len() - 1);
        let mut current = line.clone();
        loop {
            current.iter().enumerate().for_each(|(i, n)| {
                if i != 0 {
                    let prev = current[i - 1];
                    difference.push(n - prev);
                }
            });
            steps.push(difference[0]);
            if difference.iter().all(|n| *n == 0) {
                break;
            }
            current = difference.clone();
            difference.clear();
        }
        steps.reverse();
        let mut p = 0;
        steps.iter().skip(1).for_each(|n| {
            p = n - p;
        });
        result += p;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "114";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "2";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
