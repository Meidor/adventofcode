use color_eyre::eyre::Result;

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    Ok(0.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    Ok(0.to_string())
}

struct WorkflowId<'a>(&'a str);

enum Operator {
    LessThan,
    GreaterThan,
}

enum Step<'a> {
    Conditional(char, Operator, usize, WorkflowId<'a>),
    WorkflowId,
}

struct Workflow<'a> {
    id: &'a str,
    steps: Vec<Step<'a>>,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
        let expected = "0";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
    
    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "0";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
