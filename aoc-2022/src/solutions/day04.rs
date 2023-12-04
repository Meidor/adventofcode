use std::ops::RangeInclusive;
use color_eyre::eyre::Result;

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let parts: Vec<&str> = range.split('-').collect();
    let start = parts[0].parse::<u32>().unwrap();
    let end = parts[1].parse::<u32>().unwrap();
    RangeInclusive::new(start, end)
}


pub fn part_one(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|l| *l != "\n")
        .filter(|pair| {
            let ranges: Vec<RangeInclusive<u32>> =
                pair.split(',').map(parse_range).collect();
            let l = &ranges[0];
            let r = &ranges[1];
            let start_left = l.start();
            let end_left = l.end();
            let start_right = r.start();
            let end_right = r.end();
            (start_left >= start_right && end_left <= end_right)
                || (start_right >= start_left && end_right <= end_left)
        })
        .count()
        .to_string())
}


pub fn part_two(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .filter(|l| *l != "\n")
        .filter(|pair| {
            let ranges: Vec<RangeInclusive<u32>> =
                pair.split(',').map(parse_range).collect();
            let l = &ranges[0];
            let r = &ranges[1];
            let start_left = l.start();
            let end_left = l.end();
            let start_right = r.start();
            let end_right = r.end();
            l.contains(start_right)
                || l.contains(end_right)
                || r.contains(start_left)
                || r.contains(end_left)
        })
        .count()
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "2";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "4";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
