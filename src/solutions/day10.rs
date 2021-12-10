fn error_score(line: &str) -> i64 {
    let mut stack: Vec<char> = Vec::with_capacity(line.len());
    for char in line.chars() {
        match char {
            '(' => stack.push(char),
            '[' => stack.push(char),
            '{' => stack.push(char),
            '<' => stack.push(char),
            ')' => {
                if stack.pop() != Some('(') {
                    return 3;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return 57;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return 1197;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return 25137;
                }
            }
            _ => panic!("Illegal char"),
        }
    }
    0
}

fn autocomplete_score(line: &str) -> i64 {
    let mut stack: Vec<char> = Vec::with_capacity(line.len());
    for char in line.chars() {
        match char {
            '(' => stack.push(char),
            '[' => stack.push(char),
            '{' => stack.push(char),
            '<' => stack.push(char),
            ')' => {
                if stack.pop() != Some('(') {
                    return 0;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return 0;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return 0;
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return 0;
                }
            }
            _ => panic!("Illegal char"),
        }
    }
    let mut result: Vec<char> = Vec::with_capacity(stack.len());
    for char in stack {
        match char {
            '(' => result.insert(0, ')'),
            '[' => result.insert(0, ']'),
            '{' => result.insert(0, '}'),
            '<' => result.insert(0, '>'),
            _ => panic!("Illegal char"),
        }
    }
    result
        .iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Illegal char"),
        })
        .reduce(|a, b| a * 5 + b)
        .unwrap()
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    lines.iter().map(|l| error_score(l)).sum()
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|l| autocomplete_score(l))
        .filter(|f| *f != 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 26397;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 288957;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
