#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let command = parts[0];
        let amount: i64 = parts[1].parse().unwrap();
        match command {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("not suppoprted"),
        }
    }
    depth * horizontal
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut aim: i64 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let command = parts[0];
        let amount: i64 = parts[1].parse().unwrap();
        match command {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => panic!("not suppoprted"),
        }
    }
    depth * horizontal
}

#[cfg(test)]
mod test {
    fn test_input() -> Vec<String> {
        vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 150;
        let actual = crate::solutions::day02::part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 900;
        let actual = crate::solutions::day02::part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
