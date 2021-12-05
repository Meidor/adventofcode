pub const BINGO_SENTINEL: i64 = -1;

struct BingoBoard {
    pub rows: usize,
    pub columns: usize,
    pub numbers: Vec<i64>,
}

impl BingoBoard {
    pub fn check_off_number(&mut self, number: i64) {
        for i in 0..self.numbers.len() {
            if self.numbers[i] == number {
                self.numbers[i] = BINGO_SENTINEL;
            }
        }
    }

    fn has_row_bingo(&self) -> bool {
        let mut row_score = 0;
        for y in 0..self.rows {
            for x in 0..self.columns {
                row_score += self.numbers[x + y * self.columns];
            }
            if row_score == -5 {
                return true;
            }
            row_score = 0;
        }
        false
    }

    fn has_column_bingo(&self) -> bool {
        let mut column_score = 0;
        for x in 0..self.columns {
            for y in 0..self.rows {
                column_score += self.numbers[x + y * self.columns];
            }
            if column_score == -5 {
                return true;
            }
            column_score = 0;
        }
        false
    }

    pub fn get_score(&self, last_number: i64) -> i64 {
        let board_score: i64 = self.numbers.iter().filter(|n| **n != BINGO_SENTINEL).sum();
        last_number * board_score
    }

    pub fn has_bingo(&self) -> bool {
        self.has_row_bingo() || self.has_column_bingo()
    }

    pub fn print(&self) {
        for y in 0..self.rows {
            let mut line = "".to_string();
            for x in 0..self.columns {
                line += &format!("{:?} ", self.numbers[x + y * self.columns]);
            }
            println!("{:?}", line);
        }
    }
}

struct Bingo {
    pub numbers: Vec<i64>,
    pub boards: Vec<BingoBoard>,
}

fn parse_input(lines: &Vec<String>) -> Bingo {
    let mut bingo = Bingo {
        numbers: lines[0].split(",").map(|x| x.parse().unwrap()).collect(),
        boards: vec![],
    };

    let mut current_board_numbers: Vec<i64> = vec![];
    for i in 2..lines.len() {
        let line = &lines[i];
        if line == "" || i == lines.len() {
            bingo.boards.push(BingoBoard {
                rows: 5,
                columns: 5,
                numbers: current_board_numbers,
            });
            current_board_numbers = vec![];
            continue;
        }

        let numbers: Vec<i64> = line
            .split(" ")
            .filter(|x| x != &"")
            .map(|x| x.parse().unwrap())
            .collect();
        for n in numbers {
            current_board_numbers.push(n);
        }
    }
    bingo.boards.push(BingoBoard {
        rows: 5,
        columns: 5,
        numbers: current_board_numbers,
    });
    bingo
}

#[inline]
pub fn part_one(lines: &Vec<String>) -> i64 {
    let mut bingo = parse_input(lines);
    for number in bingo.numbers {
        for board in &mut bingo.boards {
            board.check_off_number(number);
            if board.has_bingo() {
                return board.get_score(number);
            }
        }
    }
    panic!("shouldn't be here...");
}

#[inline]
pub fn part_two(lines: &Vec<String>) -> i64 {
    let mut bingo = parse_input(lines);
    for number in bingo.numbers {
        for board in &mut bingo.boards {
            board.check_off_number(number);
        }
        if bingo.boards.len() > 1 {
            bingo.boards = bingo
                .boards
                .into_iter()
                .filter(|b| !b.has_bingo())
                .collect();
        } else if bingo.boards[0].has_bingo() {
            return bingo.boards[0].get_score(number);
        }
    }
    panic!("shouldn't be here...");
}

#[cfg(test)]
mod test {
    fn test_input() -> Vec<String> {
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ]
    }

    #[test]
    fn test_part_one() {
        let expected = 4512;
        let actual = crate::solutions::day04::part_one(&test_input());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 1924;
        let actual = crate::solutions::day04::part_two(&test_input());
        assert_eq!(expected, actual);
    }
}
