use std::{collections::HashMap, fmt::Display};
struct PolymerBuilder {
    iterations: usize,
    solution: Vec<char>,
    template: Vec<char>,
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
    pub fn from_input(lines: &[String], iterations: usize) -> Self {
        let mut pair_insertion = HashMap::new();
        for l in lines.iter().skip(2) {
            let x: Vec<&str> = l.split(" -> ").collect();
            pair_insertion.insert(x[0].to_string(), x[1].chars().next().unwrap());
        }

        let template: Vec<char> = lines[0].chars().collect();
        let final_length = PolymerBuilder::get_position(template.len() - 1, iterations) + 1;
        let solution: Vec<char> = vec!['.'; final_length];
        Self {
            template: lines[0].chars().collect(),
            pair_insertion,
            char_count: HashMap::new(),
            solution,
            iterations,
        }
    }

    fn get_position(position: usize, iteration: usize) -> usize {
        let a: usize = 2;
        position * a.pow(iteration as u32)
    }

    pub fn run(&mut self) {
        let mut filled = Vec::<usize>::with_capacity(self.solution.len());
        for (i, c) in self.template.iter().cloned().enumerate() {
            let pos = PolymerBuilder::get_position(i, self.iterations);
            self.solution[pos] = c;
            *self.char_count.entry(c).or_insert(0) += 1;
            filled.push(pos);
        }

        for i in 1..=self.iterations {
            for j in 0..filled.len() - 1 {
                let a = filled[j];
                let c = filled[j + 1];
                let b = (a + c) / 2;
                let new_filled: Vec<usize> = vec![];
                let new_char = *self
                    .pair_insertion
                    .get(&format!(
                        "{}{}",
                        self.solution[a],
                        self.solution[c]
                    ))
                    .unwrap();
                *self.char_count.entry(new_char).or_insert(0) += 1;
                self.solution[b] = new_char;
                filled.push(b);
            }
            filled.sort_unstable();
        }
    }

    pub fn solution(&self) -> usize {
        self.char_count.values().max().unwrap() - self.char_count.values().min().unwrap()
    }
}

#[inline]
pub fn part_one(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines, 10);
    polymer.run();
    polymer.solution().to_string()
}

#[inline]
pub fn part_two(lines: &[String]) -> String {
    let mut polymer = PolymerBuilder::from_input(lines, 40);
    polymer.run();
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
    fn test_partial() {
        let expected: Vec<char> = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().collect();
        let mut polymer = PolymerBuilder::from_input(&test_input(), 4);
        polymer.run();
        assert_eq!(expected.len(), polymer.solution.len());
        for (i, c) in expected.iter().enumerate() {
            assert_eq!(*c, polymer.solution[i]);
        }
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
