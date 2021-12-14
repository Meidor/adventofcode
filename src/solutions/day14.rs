use std::{collections::HashMap, fmt::Display};
struct PolymerBuilder {
    template: Vec<char>,
    pairs: HashMap<(char, char), usize>,
    pair_insertion: HashMap<String, char>,
    char_count: HashMap<char, usize>,
}

impl Display for PolymerBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result: String = self.template.iter().collect();
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
        let template: Vec<char> = lines[0].chars().collect();
        let pair: Vec<(char, char)> = template.windows(2).map(|w| (w[0], w[1])).collect();
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        for p in pair {
            *pairs.entry(p).or_insert(0) += 1;
        }

        let mut char_count: HashMap<char, usize> = HashMap::new();
        for c in template.iter() {
            *char_count.entry(*c).or_insert(0) += 1;
        }

        Self {
            template: lines[0].chars().collect(),
            pairs,
            pair_insertion,
            char_count
        }
    }

    fn get_position(position: usize, iteration: usize) -> usize {
        let a: usize = 2;
        position * a.pow(iteration as u32)
    }

    pub fn run(&mut self, iterations: usize) -> usize {
        
        for i in 0..iterations {
            let mut ps = HashMap::<(char, char), usize>::new();
            self.pairs.iter().for_each(|p| {
                let a = p.0 .0;
                let c = p.0 .1;
                let b = *self.pair_insertion.get(&format!("{}{}", a, c)).unwrap();
                *self.char_count.entry(b).or_insert(0) += p.1;
                *ps.entry((a, b)).or_insert(0) += p.1;
                *ps.entry((b, c)).or_insert(0) += p.1;
            });
            self.pairs = ps;
        }
        self.char_count.values().max().unwrap() - self.char_count.values().min().unwrap()
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines);
    polymer.run(10).to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines);
    polymer.run(40).to_string()
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
