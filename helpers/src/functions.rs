use num_integer::{Integer, lcm};
use num_traits::{One, Zero};
use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::ops::{Div, Mul};

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}


pub fn lcm_all<T>(input: Vec<T>) -> Option<T>
where
    T: Integer + Mul<Output = T> + Div<Output = T> + Copy + One + Zero + PartialOrd,
{
    if input.is_empty() || input.len() == 1 {
        return None;
    }

    Some(input.into_iter().fold(T::one(), |a, b| lcm(a, b)))
}