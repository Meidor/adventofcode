use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

pub trait Solution {
    fn path(&self) -> &Path;

    fn read_ints(&self) -> Result<Vec<i64>, Error> {
        let io = File::open(self.path())?;
        let br = BufReader::new(io);
        let mut v = vec![];
        for line in br.lines() {
            v.push(line?
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
        }
        Ok(v)
    }

    fn read_lines(&self) -> Result<Vec<String>, Error> {
        let io = File::open(self.path())?;
        let br = BufReader::new(io);
        let mut v: Vec<String> = vec![];
        for line in br.lines() {
            v.push(line?);
        }
        Ok(v)
    }

    fn part_one(&self) -> Result<(), Error>;
    fn part_two(&self) -> Result<(), Error>;
}
