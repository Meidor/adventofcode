use color_eyre::eyre::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

fn parse(input: &str) ->impl Iterator<Item = (Vec<Spring>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '?' => Spring::Unknown,
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                _ => unreachable!("invalid spring char"),
            })
            .collect();

        let counts: Vec<usize> = counts.split(',').filter_map(|s| s.parse().ok()).collect();

        (springs, counts)
    })
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> usize {
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<usize>>],
) -> usize {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            0
        } else {
            1
        };
    }

    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        return 0;
    }

    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }

    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }

    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(parse(input)
        .map(|(springs, counts)| count_possible_arangements(springs, counts))
        .sum::<usize>()
        .to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    Ok(parse(input)
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();
            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();

            count_possible_arangements(springs, counts)
        })
        .sum::<usize>()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
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
        let expected = "525152";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
