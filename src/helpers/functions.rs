use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_lines(path: &Path) -> Result<Vec<String>, Error> {
    let io = File::open(path)?;
    let br = BufReader::new(io);
    let mut v: Vec<String> = vec![];
    for line in br.lines() {
        v.push(line?);
    }
    Ok(v)
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
