use std::{io::Error, path::Path};

use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn path(&self) -> &Path {
        Path::new("./inputs/day1.txt")
    }

    fn part_one(&self) -> Result<(), Error>{
        for l in self.read_lines()?{
            println!("{}", l);
        }
        Ok(())
    }

    fn part_two(&self) -> Result<(), Error>{
        todo!()
    }
}

#[test]
fn test_part_one() {
}

#[test]
fn test_part_two(){
}