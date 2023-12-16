use std::{collections::HashMap, str::FromStr, sync::LazyLock};

use color_eyre::eyre::Result;
use regex::Regex;

fn hash(input: &str) -> usize {
    input.bytes().fold(0, |acc, c| {
        let mut r = acc;
        r += c as usize;
        r *= 17;
        r %= 256;
        r
    })
}

#[derive(Debug)]
enum Operation {
    Remove,
    Insert(usize),
}

#[derive(Debug)]
struct InstructionStep {
    index: usize,
    label: String,
    operation: Operation,
}

type Lens = (String, usize);

static RE: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r"([a-z]*)(-|=)([0-9]*)").unwrap());

impl FromStr for InstructionStep {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cap = RE.captures(s).unwrap();
        let label = cap[1].to_string();
        let index = hash(&label);
        let operation = match &cap[2] {
            "-" => Operation::Remove,
            "=" => Operation::Insert(cap[3].parse::<usize>()?),
            _ => unreachable!(),
        };
        Ok(InstructionStep {
            index,
            label,
            operation,
        })
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(input.trim().split(',').map(hash).sum::<usize>().to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let steps = input
        .trim()
        .split(',')
        .map(InstructionStep::from_str)
        .collect::<Result<Vec<_>>>()?;
    let mut boxes = HashMap::<usize, Vec<Lens>>::new();
    for step in steps {
        let current_box = boxes.entry(step.index).or_default();
        match step.operation {
            Operation::Insert(f) => {
                let pos = current_box.iter().position(|(l, _)| *l == step.label);
                if let Some(i) = pos {
                    current_box[i].1 = f;
                } else {
                    current_box.push((step.label, f));
                }
            }
            Operation::Remove => {
                let pos = current_box.iter().position(|(l, _)| *l == step.label);
                if let Some(i) = pos {
                    current_box.remove(i);
                }
            }
        }
    }

    Ok(boxes
        .into_iter()
        .fold(0, |acc, (k, v)| {
            acc + (k + 1)
                * v.into_iter()
                    .enumerate()
                    .fold(0, |acc, (i, (_label, f))| acc + (i + 1) * f)
        })
        .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "1320";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "145";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
