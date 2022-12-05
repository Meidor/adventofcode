use std::collections::HashMap;
use regex::Regex;

fn parse_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let count = lines.len();
    let columns = &lines[0..count - 1];
    let stack_amount = columns[0].len().div_ceil(4);
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::with_capacity(stack_amount);
    for i in 0..stack_amount {
        stacks.insert(i + 1, vec![]);
    }
    for line in lines.iter().rev() {
        for i in 0..stack_amount {
            let chr = line.chars().nth(1 + i * 4).unwrap();
            if chr >= 'A' && chr <= 'Z' {
                stacks.get_mut(&(i + 1)).unwrap().push(chr);
            }
        }
    }
    stacks
}

#[inline]
pub fn part_one(input: &str) -> String {
    let move_re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(parts[0]);
    parts[1].lines().for_each(|instruction| {
        let groups = move_re.captures(instruction).unwrap();
        let amount = groups[1].parse::<usize>().unwrap();
        let from = groups[2].parse::<usize>().unwrap();
        let to = groups[3].parse::<usize>().unwrap();
        let [from , to] = stacks.get_many_mut([&from, &to]).unwrap();
        for i in 0..amount {
            to.push(from.pop().unwrap());
        }
    });

    let columns = *stacks.keys().max().unwrap();
    let mut result = String::new();
    for i in 1..=columns {
        result += &format!("{}", stacks.get_mut(&i).unwrap().pop().unwrap());
    }
    result
}

#[inline]
pub fn part_two(input: &str) -> String {
    let move_re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(parts[0]);
    parts[1].lines().for_each(|instruction| {
        let groups = move_re.captures(instruction).unwrap();
        let amount = groups[1].parse::<usize>().unwrap();
        let from = groups[2].parse::<usize>().unwrap();
        let to = groups[3].parse::<usize>().unwrap();
        let [from , to] = stacks.get_many_mut([&from, &to]).unwrap();
        
        let mut letters: Vec<char> = vec![];
        for i in 0..amount {
            letters.push(from.pop().unwrap());
        }
        letters.reverse();
        for letter in letters {
            to.push(letter);
        }
    });

    let columns = *stacks.keys().max().unwrap();
    let mut result = String::new();
    for i in 1..=columns {
        result += &format!("{}", stacks.get_mut(&i).unwrap().pop().unwrap());
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
    }

    #[test]
    fn test_part_one() {
        let expected = "CMZ";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "MCD";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
