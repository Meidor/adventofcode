use std::str::FromStr;

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
