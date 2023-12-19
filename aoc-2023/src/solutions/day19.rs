use color_eyre::eyre::Result;
use regex::Regex;
use std::{collections::HashMap, str::FromStr, sync::LazyLock};

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-z]*)(<|>)([0-9]*):([a-zA-Z]*)").unwrap());

#[derive(Debug, Clone)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl Operator {
    fn invert(&self) -> Self {
        match self {
            Operator::LessThan => Operator::GreaterThan,
            Operator::GreaterThan => Operator::LessThan,
        }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Conditional(char, Operator, usize, String),
    GoTo(String),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_workflow(input: &str) -> HashMap<String, Vec<Rule>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (id, rest) = l.split_once("{").unwrap();
            let id = id.to_string();
            let rest = rest.trim_end_matches("}");
            let rules = rest
                .split(',')
                .map(|r| {
                    if r.contains("<") || r.contains(">") {
                        let captures = RE.captures(r).unwrap();
                        let part = captures[1].parse::<char>().unwrap();
                        let operator = match &captures[2] {
                            "<" => Operator::LessThan,
                            ">" => Operator::GreaterThan,
                            _ => unreachable!("Invalid operator: {}", &captures[2]),
                        };
                        let value = usize::from_str(&captures[3]).unwrap();
                        let goto = captures[4].to_string();
                        Rule::Conditional(part, operator, value, goto)
                    } else {
                        Rule::GoTo(r.to_string())
                    }
                })
                .collect();

            (id, rules)
        })
        .collect::<HashMap<String, Vec<Rule>>>()
}

fn parse_parts(input: &str) -> Vec<Part> {
    input
        .trim()
        .lines()
        .map(|l| {
            let xmas = l[1..l.len() - 1]
                .split(',')
                .map(|p| {
                    let mut split = p.split('=');
                    split.next().unwrap();
                    let value = split.next().unwrap();
                    value
                })
                .collect::<Vec<&str>>();
            Part {
                x: usize::from_str(xmas[0]).unwrap(),
                m: usize::from_str(xmas[1]).unwrap(),
                a: usize::from_str(xmas[2]).unwrap(),
                s: usize::from_str(xmas[3]).unwrap(),
            }
        })
        .collect()
}

fn process_parts(parts: Vec<Part>, workflow: HashMap<String, Vec<Rule>>) -> Vec<Part> {
    parts
        .into_iter()
        .filter_map(|p| {
            let mut current_workflow = workflow.get("in").unwrap();
            let mut current_step_idx = 0;
            let mut current_step = current_workflow[current_step_idx].clone();
            loop {
                if let Rule::GoTo(id) = current_step {
                    if id == "A" {
                        return Some(p);
                    }
                    if id == "R" {
                        return None;
                    }
                    let id = id.to_string();

                    current_workflow = workflow.get(&id).unwrap();
                    current_step_idx = 0;
                    current_step = current_workflow[current_step_idx].clone();
                }
                if let Rule::Conditional(part, operator, value, id) = current_step {
                    let part_value = match part {
                        'x' => p.x,
                        'm' => p.m,
                        'a' => p.a,
                        's' => p.s,
                        _ => unreachable!("Invalid part: {}", part),
                    };
                    match operator {
                        Operator::LessThan => {
                            if part_value < value {
                                if id == "A" {
                                    return Some(p);
                                }
                                if id == "R" {
                                    return None;
                                }
                                current_workflow = workflow.get(&id).unwrap();
                                current_step_idx = 0;
                                current_step = current_workflow[current_step_idx].clone();
                            } else {
                                current_step_idx += 1;
                                current_step = current_workflow[current_step_idx].clone();
                            }
                        }
                        Operator::GreaterThan => {
                            if part_value > value {
                                if id == "A" {
                                    return Some(p);
                                }
                                if id == "R" {
                                    return None;
                                }
                                current_workflow = workflow.get(&id).unwrap();
                                current_step_idx = 0;
                                current_step = current_workflow[current_step_idx].clone();
                            } else {
                                current_step_idx += 1;
                                current_step = current_workflow[current_step_idx].clone();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (workflow_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflow = parse_workflow(workflow_str);
    let parts = parse_parts(parts_str);
    (workflow, parts)
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let (workflow, parts) = parse_input(input);
    let accepted_parts = process_parts(parts, workflow);
    let result = accepted_parts
        .iter()
        .map(|p| p.x + p.m + p.a + p.s)
        .sum::<usize>();
    Ok(result.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let (workflow, _) = parse_input(input);
    let mut x = 4000;
    let mut m = 4000;
    let mut a = 4000;
    let mut s = 4000;

    let mut current_workflow = workflow.get("in").unwrap();
    let mut current_step_idx = 0;
    let mut current_step = current_workflow[current_step_idx].clone();

    if let Rule::Conditional(part, operator, value, id) = current_step {}

    Ok(0.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "19114";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "167409079868000";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
