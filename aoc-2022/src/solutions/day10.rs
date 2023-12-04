use color_eyre::eyre::Result;

fn get_values(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|l| *l != "\n")
        .flat_map(|l| match l {
            "noop" => vec![0],
            _ => {
                let value = l
                    .split(' ')
                    .nth(1)
                    .expect("invalid input")
                    .parse::<i32>()
                    .expect("invalid input");
                vec![0, value]
            }
        })
        .collect::<Vec<i32>>()
}

pub fn part_one(input: &str) -> Result<String> {
    let values = get_values(input);
    let mut x = 1;
    let mut sum = 0;
    let mut cycle = 1;
    for i in values {
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
        x += i;
        cycle += 1;
    }
    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    let values = get_values(input);
    let mut x = 1;
    let mut cycle = 1;
    let crt_width = 40;
    let crt_height = 6;
    let crt_size = crt_width * crt_height;
    let mut output = String::new();
    output += "\n";
    output += "```\n";
    for i in values {
        let crt_i = (cycle - 1) % crt_size;
        let crt_column = crt_i % 40;
        if crt_column >= x - 1 && crt_column <= x + 1 {
            output += "▓"
        } else {
            output += "░";
        }
        if crt_column == 39 {
            output += "\n";
        }
        x += i;
        cycle += 1;
    }
    output += "```\n";
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "13140";
        let actual = part_one(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "
```
▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░▓▓░░
▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░░░▓▓▓░
▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░▓▓▓▓░░░░
▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░▓▓▓▓▓░░░░░
▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓▓▓░░░░░░▓▓▓▓
▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░░░▓▓▓▓▓▓▓░░░░░
```
";
        let actual = part_two(test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
