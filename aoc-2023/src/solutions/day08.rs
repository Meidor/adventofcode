use std::collections::HashMap;

use color_eyre::eyre::Result;
use helpers::lcm_all;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct RouteChoice<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_directions(input: &str) -> (&str, HashMap<&str, RouteChoice<'_>>) {
    let (instructions, graph_str) = input.split_once("\n\n").unwrap();
    let mut directions = HashMap::<&str, RouteChoice>::new();
    for line in graph_str.lines() {
        let (id, children) = line.split_once(" = ").unwrap();
        let children = children
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let (left, right) = children.split_once(", ").unwrap();
        directions.insert(id, RouteChoice { id, left, right });
    }
    (instructions, directions)
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let (instructions, directions) = parse_directions(input);
    let mut node: &RouteChoice = directions.get("AAA").unwrap();
    let mut steps = 0;
    let chars: Vec<_> = instructions.trim().chars().collect();
    let mut i = 0;
    loop {
        let instruction = chars[i];
        match instruction {
            'L' => {
                node = directions.get(node.left).unwrap();
            }
            'R' => {
                node = directions.get(node.right).unwrap();
            }
            _ => unreachable!(),
        }

        steps += 1;
        if node.id == "ZZZ" {
            return Ok(steps.to_string());
        }

        i += 1;
        i %= chars.len();
    }
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let (instructions, directions) = parse_directions(input);
    let mut nodes = directions
        .keys()
        .filter(|id| id.ends_with('A'))
        .map(|id| directions.get(id).unwrap())
        .collect::<Vec<&RouteChoice>>();

    let mut path_lengths = vec![0; nodes.len()];

    let mut steps = 0u64;
    let chars: Vec<_> = instructions.trim().chars().collect();
    let mut i = 0;

    while path_lengths.iter().any(|x| *x == 0) {
        let instruction = chars[i];

        for (i, node) in nodes.iter_mut().enumerate() {
            match instruction {
                'L' => {
                    *node = directions.get(node.left).unwrap();
                }
                'R' => {
                    *node = directions.get(node.right).unwrap();
                }
                _ => unreachable!(),
            }

            if node.id.ends_with('Z') && path_lengths[i] == 0 {
                path_lengths[i] = steps + 1;
            }
        }
        steps += 1;
        i += 1;
        i %= chars.len();
    }

    let result = lcm_all(path_lengths).unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
    }

    fn test_input_2() -> &'static str {
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "6";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "6";
        let actual = part_two(&test_input_2())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
