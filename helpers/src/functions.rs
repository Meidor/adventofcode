use std::collections::HashSet;

use glam::IVec2;

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn distance(a: IVec2, b: IVec2) -> usize {
    let md = (a - b).abs();
    (md.x + md.y) as usize
}
