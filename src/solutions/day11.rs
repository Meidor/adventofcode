use std::collections::{HashMap, VecDeque};

use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Mul(u64),
    MulOld,
    Add(u64),
    AddOld,
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        let op = op.trim().replace("Operation: new = old ", "");
        let op_parts = op.split(" ").collect::<Vec<&str>>();
        let op = op_parts[0];
        let val = op_parts[1];
        match op {
            "*" => {
                if val == "old" {
                    Operation::MulOld
                } else {
                    Operation::Mul(val.parse().expect("invalid input"))
                }
            }
            "+" => {
                if val == "old" {
                    Operation::AddOld
                } else {
                    Operation::Add(val.parse().expect("invalid input"))
                }
            }
            _ => unreachable!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct Test {
    div_value: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let id_line = lines[0];
        let id = id_line[id_line.len() - 2..id_line.len() - 1]
            .parse::<usize>()
            .expect("invalid input");
        let items_line = lines[1];
        let items: VecDeque<u64> = items_line
            .trim()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|wi| -> u64 { wi.parse().expect("invalid input") })
            .collect();
        let operation = Operation::from(lines[2]);
        let div_line = lines[3];
        let true_line = lines[4];
        let false_line = lines[5];
        let div_value: u64 = div_line
            .trim()
            .replace("Test: divisible by ", "")
            .parse()
            .expect("invalid input");
        let if_true = true_line[true_line.len() - 1..true_line.len()]
            .parse::<usize>()
            .expect("invalid input");
        let if_false = false_line[false_line.len() - 1..false_line.len()]
            .parse::<usize>()
            .expect("invalid input");
        Monkey {
            id,
            items,
            operation,
            test: Test {
                div_value,
                if_true,
                if_false,
            },
            inspections: 0,
        }
    }
}

fn do_monkey_business(input: &str, iterations: usize, is_part_two: bool) -> Result<String> {
    let monkey_inputs = input.split("\n\n");
    let mut monkeys = monkey_inputs
        .map(|mi| {
            let m = Monkey::from(mi);
            let mi = m.id;
            (mi, m)
        })
        .collect::<HashMap<usize, Monkey>>();
    let reducer: u64 = monkeys.values().map(|m| m.test.div_value).product();
    for i in 1..=iterations {
        for mi in 0..monkeys.keys().len() {
            let mut items = monkeys.get(&mi).expect("no monkey").items.clone();
            while let Some(item) = items.pop_front() {
                let mut worry = item;
                worry = match monkeys.get(&mi).expect("no monkey").operation {
                    Operation::Mul(x) => worry * x,
                    Operation::MulOld => worry * worry,
                    Operation::Add(x) => worry + x,
                    Operation::AddOld => worry + worry,
                };
                if !is_part_two {
                    worry = worry.div_floor(3);
                }
                worry %= reducer;
                let receiver_id =
                    if worry % monkeys.get(&mi).expect("no monkey").test.div_value == 0 {
                        monkeys.get(&mi).expect("no monkey").test.if_true
                    } else {
                        monkeys.get(&mi).expect("no monkey").test.if_false
                    };

                let [giver, receiver] = monkeys
                    .get_many_mut([&mi, &receiver_id])
                    .expect("no monkey for you");
                giver.inspections += 1;
                receiver.items.push_back(worry);
                giver.items = items.clone();
            }
        }
    }

    let monkey_business: u64 = monkeys
        .values()
        .map(|m| m.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product();
    Ok(monkey_business.to_string())
}

pub fn part_one(input: &str) -> Result<String> {
    do_monkey_business(input, 20, false)
}

pub fn part_two(input: &str) -> Result<String> {
    do_monkey_business(input, 10000, true)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "10605";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "2713310158";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
