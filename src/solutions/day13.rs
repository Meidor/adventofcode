use std::{cmp::Ordering, fmt::Display, iter::Peekable};

use color_eyre::eyre::Result;
use log::info;

use crate::helpers::{LexError, LexResult, ParseError, ParseResult, Parser, Token};

#[derive(Debug, Copy, Clone, PartialEq)]
enum LexTokenType {
    OpenBracket,
    CloseBracket,
    Integer,
}

#[derive(Debug, Clone, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(i32),
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        is_pair_correct(self, other) == Ordering::Equal
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(is_pair_correct(self, other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        is_pair_correct(self, other)
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketData::List(d) => {
                let mut s = String::new();
                s += "[";
                for pd in d {
                    s += &pd.to_string();
                    s += ",";
                }
                s = s.trim_end_matches(",").to_string();
                s += "]";
                write!(f, "{}", s)
            }
            PacketData::Integer(i) => write!(f, "{}", i),
        }
    }
}

struct PacketParser {
    tokens: Vec<Token<LexTokenType>>,
    pos: usize,
}

/*
*    GRAMMAR:
*    List ::= '[' ((List | Integer) ','?)* ']'
*    Integer::= #'[0-9]+'
*/
impl PacketParser {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: Self::lex(input).expect("error while lexing"),
            pos: 0,
        }
    }

    fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> i32 {
        let mut number = c
            .to_string()
            .parse::<i32>()
            .expect("The caller should have passed a digit.");
        while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i32>()) {
            number = number * 10 + digit;
            iter.next();
        }
        number
    }

    fn lex(input: &str) -> LexResult<Vec<Token<LexTokenType>>> {
        let mut result = Vec::new();
        let mut it = input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => {
                    it.next();
                    let n = Self::get_number(c, &mut it);
                    let content = n.to_string();
                    result.push(Token::new(LexTokenType::Integer, content))
                }
                '[' => {
                    let content = c.to_string();
                    result.push(Token::new(LexTokenType::OpenBracket, content));
                    it.next();
                }
                ']' => {
                    let content = c.to_string();
                    result.push(Token::new(LexTokenType::CloseBracket, content));
                    it.next();
                }
                ',' => {
                    it.next();
                }
                _ => {
                    return Err(LexError::InvalidCharacter(c));
                }
            }
        }
        Ok(result)
    }

    pub fn parse(&mut self) -> ParseResult<PacketData, LexTokenType> {
        self.parse_list()
    }

    fn parse_list(&mut self) -> ParseResult<PacketData, LexTokenType> {
        let mut packet_data: Vec<PacketData> = vec![];
        if self.pos == 0 {
            self.advance();
        }
        loop {
            if self.is_eof() {
                break;
            }

            if self.is_match(LexTokenType::OpenBracket) {
                self.advance();
                let sub_list = self.parse_list().unwrap();
                packet_data.push(sub_list);
            }

            if self.is_match(LexTokenType::Integer) {
                packet_data.push(PacketData::Integer(self.parse_integer().unwrap()));
            }

            if self.is_match(LexTokenType::CloseBracket) {
                self.advance();
                return Ok(PacketData::List(packet_data));
            }
        }
        Ok(PacketData::List(packet_data))
    }

    fn parse_integer(&mut self) -> ParseResult<i32, LexTokenType> {
        let token = self.peek();
        if self.is_match(LexTokenType::Integer) {
            let result = token
                .content
                .parse::<i32>()
                .map_err(|_| ParseError::InvalidInteger);
            self.advance();
            return result;
        }
        Err(ParseError::UnexpectedToken(
            LexTokenType::Integer,
            token.token_type,
        ))
    }
}

impl Parser<LexTokenType> for PacketParser {
    fn tokens(&self) -> &Vec<Token<LexTokenType>> {
        &self.tokens
    }

    fn pos(&self) -> usize {
        self.pos
    }

    fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }
}

fn get_packet_pairs(input: &str) -> Vec<(PacketData, PacketData)> {
    input
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|pp| {
            let lines: Vec<&str> = pp.lines().collect();
            let mut parser_one = PacketParser::new(lines[0]);
            let packet_one = parser_one.parse().expect("invalid packet");
            let mut parser_two = PacketParser::new(lines[1]);
            let packet_two = parser_two.parse().expect("invalid packet");
            (packet_one, packet_two)
        })
        .collect()
}

fn is_pair_correct(left: &PacketData, right: &PacketData) -> Ordering {
    info!("comparing {} vs {}", left, right);
    let debug_left = left.to_string();
    let debug_right = right.to_string();
    if let PacketData::Integer(l) = left {
        if let PacketData::Integer(r) = right {
            if l < r {
                return Ordering::Less;
            }
            if l > r {
                return Ordering::Greater;
            }
            return Ordering::Equal;
        } else {
            return is_pair_correct(&PacketData::List(vec![left.clone()]), right);
        }
    }

    if let PacketData::List(l) = left {
        if let PacketData::List(r) = right {
            let mut li = l.into_iter();
            let mut ri = r.into_iter();
            loop {
                let next_left = li.next();
                let next_right = ri.next();

                if next_left.is_none() && next_right.is_none() {
                    return Ordering::Equal;
                }
                if next_left.is_none() {
                    return Ordering::Less;
                }
                if next_right.is_none() {
                    return Ordering::Greater;
                }
                let next_left = next_left.unwrap();
                let next_right = next_right.unwrap();
                match is_pair_correct(next_left, next_right) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    _ => {}
                }
            }
        } else {
            return is_pair_correct(left, &PacketData::List(vec![right.clone()]));
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Result<String> {
    let pairs = get_packet_pairs(input);
    let mut result = 0;
    for i in 0..pairs.len() {
        let (left, right) = &pairs[i];
        info!("START COMPARING PAIR {} ({}, {})", i, left, right);
        let ordering = is_pair_correct(left, right);
        match ordering {
            Ordering::Less => result += i + 1,
            _ => {}
        };
    }
    Ok(result.to_string())
}

pub fn part_two(input: &str) -> Result<String> {
    let mut packets: Vec<PacketData> = input
        .trim()
        .lines()
        .filter(|l| *l != "")
        .map(|p| PacketParser::new(p).parse().unwrap())
        .collect();
    let sentinel_1 = PacketParser::new("[[2]]").parse().unwrap();
    let sentinel_2 = PacketParser::new("[[6]]").parse().unwrap();
    packets.push(sentinel_1.clone());
    packets.push(sentinel_2.clone());
    packets.sort();
    let mut result = 1;
    for i in 0..packets.len() {
        if packets[i].eq(&sentinel_1) || packets[i].eq(&sentinel_2) {
            result *= i + 1;
        }
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn test_input() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"
    }

    fn test_input_two() -> &'static str {
        "[[2],[7]]
[[2,6]]

[[],[2,7]]
[[2],[6]]

[2,7]
[2,[6]]
"
    }

    #[test]
    fn test_part_one() -> Result<()> {
        init();
        let expected = "13";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        init();
        let expected = "140";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_parse_packets() -> Result<()> {
        let lines: Vec<&str> = test_input().trim().lines().filter(|l| *l != "").collect();
        for expected in lines {
            let mut parser = PacketParser::new(expected);
            let actual = parser.parse().unwrap().to_string();
            assert_eq!(expected, actual);
        }
        Ok(())
    }
}
