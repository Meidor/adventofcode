use itertools::Itertools;

#[inline]
pub fn part_one(input: &str) -> String {
    input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .filter(|snack| *snack != "")
                .map(|snack| snack.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
        .to_string()
}

#[inline]
pub fn part_two(input: &str) -> String {
    let sums: Vec<u64> = input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .filter(|snack| *snack != "")
                .map(|snack| snack.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted_unstable()
        .collect();
    let amount = sums.len();
    (sums[amount - 3] + sums[amount - 2] + sums[amount - 1]).to_string()
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
    fn test_part_one() {
        let expected = "24000";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "45000";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
