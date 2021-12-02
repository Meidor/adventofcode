use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines(path: &Path) -> Result<Vec<String>, Error> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut v: Vec<String> = vec![];
    for line in br.lines() {
        v.push(line?);
    }
    Ok(v)
}


pub fn parse_input<T>(input: &Vec<String>) -> Vec<T> where T: FromStr + Default {
    input
        .iter()
        .map(|i| i.parse().unwrap_or_default())
        .collect()
}