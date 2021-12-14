use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
struct PolymerBuilder {
    polymer: Vec<char>,
    pair_insertion: HashMap<String, char>,
}

impl Display for PolymerBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: String = self.polymer.iter().collect();
        write!(f, "{}", result)
    }
}

impl PolymerBuilder {
    pub fn from_input(lines: &[String]) -> Self {
        let mut pair_insertion = HashMap::new();
        for l in lines.iter().skip(2) {
            let x: Vec<&str> = l.split(" -> ").collect();
            pair_insertion.insert(x[0].to_string(), x[1].chars().next().unwrap());
        }
        Self {
            polymer: lines[0].chars().collect(),
            pair_insertion,
        }
    }

    fn step(&mut self) {
        let a = self.polymer.iter().cloned();
        let mut result: Vec<char> = Vec::with_capacity(self.polymer.len() + self.polymer.len() - 1);
        for i in 0..self.polymer.len() - 1 {
            let a = self.polymer[i];
            let c = self.polymer[i + 1];
            let b = *self.pair_insertion.get(&format!("{}{}", a, c)).unwrap();
            result.push(a);
            result.push(b);
        }
        result.push(*self.polymer.last().unwrap());
        self.polymer = result;
    }

    pub fn run(&mut self, steps: usize) {
        for i in 0..steps {
            println!("step {}", i);
            self.step();
        }
    }

    pub fn solution(&self) -> usize {
        let unique: HashSet<char> = HashSet::from_iter(self.polymer.iter().cloned());
        let mut counts: Vec<usize> = vec![];
        for i in unique {
            let count = self.polymer.iter().filter(|f| **f == i).count();
            counts.push(count);
        }
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines);
    polymer.run(10);
    polymer.solution().to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines);
    polymer.run(40);
    polymer.solution().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = "1588";
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = "2188189693529";
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
