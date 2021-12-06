struct School([usize; 9]);

impl School {
    pub fn new(fishes: Vec<usize>) -> Self {
        let mut school: [usize; 9] = [0; 9];
        for fish in fishes {
            school[fish] += 1;
        }
        Self { 0: school }
    }

    pub fn simulate(&mut self, days: usize) {
        for _ in 0..days {
            self.step();
        }
    }

    fn step(&mut self) {
        let parents = self.0[0];
        self.0[0] = 0;
        for i in 1..=8 {
            self.0[i - 1] = self.0[i];
        }
        self.0[6] += parents;
        self.0[8] = parents;
    }

    pub fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

fn parse_input(lines: &[String]) -> School {
    let fishes: Vec<usize> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|a| a.parse().unwrap())
        .collect();
    School::new(fishes)
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let mut school = parse_input(lines);
    school.simulate(80);
    school.count() as i64
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let mut school = parse_input(lines);
    school.simulate(256);
    school.count() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec!["3,4,3,1,2".to_string()]
    }

    #[test]
    fn test_part_one() {
        let expected = 5934;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 26984457539;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
