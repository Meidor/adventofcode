use std::collections::HashMap;
const SEGMENT_SENTINEL: Segment = Segment('.');

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Segment(char);

#[derive(Clone, Eq, PartialEq)]
struct Digit {
    segments: [Segment; 7],
}


impl Digit {
    fn filter_segments(&self, segments: &[&Segment]) -> Vec<&Segment> {
        self.get_significant_segments()
            .into_iter()
            .filter(|f| !segments.contains(f))
            .collect()
    }

    fn get_significant_segments(&self) -> Vec<&Segment> {
        self.segments
            .iter()
            .filter(|s| s.0 != SEGMENT_SENTINEL.0)
            .collect()
    }

    pub fn segment_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|f| f.0 != SEGMENT_SENTINEL.0)
            .count()
    }

    pub fn from_string(input: &str) -> Self {
        let mut segments = [SEGMENT_SENTINEL; 7];
        for (i, c) in input.chars().enumerate() {
            segments[i] = Segment(c);
        }
        Self { segments }
    }

    pub fn get_string(&self) -> String {
        let mut segs = self.get_significant_segments();
        segs.sort_unstable();
        segs.iter().map(|s| s.0).collect()
    }

    pub fn has_segment(&self, segment: &Segment) -> bool {
        self.segments.contains(segment)
    }
}

struct Display {
    input_digits: Vec<Digit>,
    output_digits: Vec<Digit>,
    decode_map: Option<HashMap<String, u8>>,
}

impl Display {
    pub fn new(input_digits: Vec<Digit>, output_digits: Vec<Digit>) -> Self {
        let mut res = Self {
            input_digits,
            output_digits,
            decode_map: None,
        };
        res.decode();
        res
    }

    fn digit_by_segment_count(&self, count: usize) -> &Digit {
        self.input_digits
            .iter()
            .filter(|d| d.segment_count() == count)
            .collect::<Vec<&Digit>>()
            .first()
            .unwrap()
    }

    fn filter_digits(&self, digits: &[&Digit]) -> Vec<&Digit> {
        self.input_digits
            .iter()
            .filter(|d| !digits.contains(d))
            .collect()
    }

    fn decode(&mut self) {
        let one = self.digit_by_segment_count(2);
        let four = self.digit_by_segment_count(4);
        let seven = self.digit_by_segment_count(3);
        let eight = self.digit_by_segment_count(7);

        let one_segments = one.get_significant_segments();
        let segment_a = *seven.filter_segments(&one_segments).first().unwrap();
        let one_a = one_segments[0];
        let one_b = one_segments[1];
        let a_count = self
            .input_digits
            .iter()
            .map(|d| d.has_segment(one_a))
            .filter(|f| *f)
            .count();
        let b_count = self
            .input_digits
            .iter()
            .map(|d| d.has_segment(one_b))
            .filter(|f| *f)
            .count();

        let mut c: Option<&Segment> = None;
        let mut f: Option<&Segment> = None;
        if a_count == 9 && b_count == 8 {
            c = Some(one_b);
            f = Some(one_a);
        } else if a_count == 8 && b_count == 9 {
            c = Some(one_a);
            f = Some(one_b);
        }
        let segment_c = c.unwrap();
        let segment_f = f.unwrap();

        let mut tw: Option<&Digit> = None;
        let mut t: Option<&Digit> = None;
        let mut f: Option<&Digit> = None;
        let mut s: Option<&Digit> = None;
        let unknown_digits = self.filter_digits(&[one, four, seven, eight]);
        for digit in unknown_digits {
            let filtered = digit.filter_segments(&[segment_a, segment_c, segment_f]);
            if filtered.len() == 2 {
                t = Some(digit);
            } else if filtered.len() == 4 {
                s = Some(digit);
            } else if !digit.has_segment(segment_c) {
                f = Some(digit);
            } else if digit.get_significant_segments().len() == 5 {
                tw = Some(digit);
            }
        }
        let two = tw.unwrap();
        let three = t.unwrap();
        let five = f.unwrap();
        let six = s.unwrap();
        let unknown_digits = self.filter_digits(&[one, two, three, four, five, six, seven, eight]);

        let zero_nine: Vec<Vec<&Segment>> = unknown_digits
            .iter()
            .map(|d| d.filter_segments(&[segment_a, segment_c, segment_f]))
            .collect();

        let zero_nine_a = &zero_nine[0];
        let zero_nine_b = &zero_nine[1];
        let difference_a: Vec<_> = zero_nine_a
            .iter()
            .filter(|item| !zero_nine_b.contains(item))
            .collect();
        let difference_b: Vec<_> = zero_nine_b
            .iter()
            .filter(|item| !zero_nine_a.contains(item))
            .collect();

        let seg_a = **difference_a.first().unwrap();
        let seg_b = **difference_b.first().unwrap();

        let mut n: Option<&Digit> = None;
        let mut z: Option<&Digit> = None;
        if four.has_segment(seg_a) && !four.has_segment(seg_b) {
            n = Some(unknown_digits[0]);
            z = Some(unknown_digits[1]);
        } else if four.has_segment(seg_b) && !four.has_segment(seg_a) {
            z = Some(unknown_digits[0]);
            n = Some(unknown_digits[1]);
        }
        let nine = n.unwrap();
        let zero = z.unwrap();

        let mut map: HashMap<String, u8> = HashMap::new();
        map.insert(zero.get_string(), 0);
        map.insert(one.get_string(), 1);
        map.insert(two.get_string(), 2);
        map.insert(three.get_string(), 3);
        map.insert(four.get_string(), 4);
        map.insert(five.get_string(), 5);
        map.insert(six.get_string(), 6);
        map.insert(seven.get_string(), 7);
        map.insert(eight.get_string(), 8);
        map.insert(nine.get_string(), 9);
        self.decode_map = Some(map);
    }

    pub fn calculate_value(&self) -> i64 {
        let map = self.decode_map.as_ref().unwrap();
        let digits: Vec<u8> = self
            .output_digits
            .iter()
            .map(|d| *map.get(&d.get_string()).unwrap_or(&99))
            .collect();
        let thousands = digits[0] as i64 * 1000;
        let hundreds = digits[1] as i64 * 100;
        let tens = digits[2] as i64 * 10;
        let ones = digits[3] as i64;
        thousands + hundreds + tens + ones
    }
}

fn parse_input(lines: &[String]) -> Vec<Display> {
    lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(" | ").collect();
            let input_part = parts[0];
            let output_part = parts[1];
            let input_digits: Vec<Digit> = input_part.split(' ').map(Digit::from_string).collect();
            let output_digits: Vec<Digit> =
                output_part.split(' ').map(Digit::from_string).collect();
            Display::new(input_digits, output_digits)
        })
        .collect()
}

#[inline]
pub fn part_one(lines: &[String]) -> i64 {
    let displays = parse_input(lines);
    let unique_segment_count = [2, 4, 3, 7];
    displays
        .into_iter()
        .flat_map(|d| d.output_digits)
        .filter(|d| unique_segment_count.contains(&d.segment_count()))
        .count() as i64
}

#[inline]
pub fn part_two(lines: &[String]) -> i64 {
    let displays = parse_input(lines);
    displays.into_iter().map(|d| d.calculate_value()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<String> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 26;
        let actual = part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 61229;
        let actual = part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
