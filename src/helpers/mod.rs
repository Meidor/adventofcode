use std::str::FromStr;
mod functions;
mod graph;
mod grid;
mod parser;
mod tree;

pub use functions::*;
pub use graph::*;
pub use grid::*;
pub use parser::*;
pub use tree::*;

pub trait InputHelpers {
    fn parse_input<T: FromStr>(&self) -> Result<Vec<T>, T::Err>;
    fn reverse_string(&self) -> String;
}

impl InputHelpers for &str {
    #[inline(always)]
    fn parse_input<T: FromStr>(&self) -> Result<Vec<T>, T::Err> {
        self.trim().lines().map(|line| line.parse()).collect()
    }

    #[inline(always)]
    fn reverse_string(&self) -> String {
        self.chars().rev().collect()
    }
}
