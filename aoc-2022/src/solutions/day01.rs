use itertools::Itertools;
use color_eyre::eyre::Result;

pub fn part_one(input: &str) -> Result<String> {
    let result = input
    .split("\n\n")
    .map(|elf| {
        elf.split('\n')
            .filter(|snack| !snack.is_empty())
            .map(|snack| snack.parse::<u64>().expect("can't parse snack amount"))
            .sum::<u64>()
    })
    .max()
    .expect("no max found")
    .to_string();
    Ok(result)
}


pub fn part_two(input: &str) -> Result<String> {
    let sums: Vec<u64> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter(|snack| !snack.is_empty())
                .map(|snack| snack.parse::<u64>().expect("can't parse snack amount"))
                .sum::<u64>()
        })
        .sorted_unstable()
        .collect();
    let amount = sums.len();
    Ok((sums[amount - 3] + sums[amount - 2] + sums[amount - 1]).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "24000";
        let actual: &str = &part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "45000";
        let actual: &str = &part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
