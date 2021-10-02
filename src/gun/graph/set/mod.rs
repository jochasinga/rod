use bincode;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Set<T> is the base set wrapping the HashSet primitive and
/// providing certain set abstractions.
/// WARNING: Casting Vertex<T> or Edge<T> to Set<T> are lossy
/// meaning once the casting is done there is no way of differentiating
/// a Set<T> if it was previously of type Vertex<T> or Edge<T>
/// aside from its size. Therefore, this casting should be limited
/// to just within the implementation of methods of Vertex<T> and Edge<T> only.

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Set<T: Eq + Hash + fmt::Debug>(pub HashSet<T>);

impl<T> Hash for Set<T>
where
    T: Eq + Hash + Serialize + fmt::Debug,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        encoded.hash(state);
    }
}

impl<T> PartialEq for Set<T>
where
    T: Eq + Hash + Clone + fmt::Debug,
{
    fn eq(&self, other: &Set<T>) -> bool {
        let (Set(h1), Set(h2)) = (&self, other);
        h1.difference(h2).count() == 0
    }
}

impl<T> Eq for Set<T> where T: Eq + Hash + Clone + fmt::Debug {}

impl<T> Set<T>
where
    T: Eq + Hash + Clone + fmt::Debug,
{
    pub fn new() -> Self {
        Set(HashSet::new())
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> {
        let Set(h) = self;
        h.iter()
    }

    pub fn insert(&mut self, data: T) {
        let Set(inner) = self;
        inner.insert(data);
        let new_inner = inner.clone();
        *self = Set(new_inner);
    }

    pub fn len(&self) -> usize {
        let Set(hs) = self;
        hs.len()
    }

    pub fn is_subset(&self, other: &Set<T>) -> bool {
        let (Set(h1), Set(h2)) = (self, other);
        h1.is_subset(h2)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let intersection = self.0.intersection(&other.0);
        let mut hs = HashSet::new();
        for s in intersection.into_iter() {
            let s_ = s.clone();
            hs.insert(s_);
        }
        Set(hs)
    }
}

impl<'a, T> IntoIterator for &'a Set<T>
where
    T: Eq + Hash + Clone + fmt::Debug,
{
    type Item = &'a T;
    type IntoIter = std::collections::hash_set::Iter<'a, T>;
    fn into_iter(self) -> std::collections::hash_set::Iter<'a, T> {
        let Set(hs) = self;
        hs.iter()
    }
}

impl<T> From<HashSet<T>> for Set<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(h: HashSet<T>) -> Self {
        Set(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_set_is_empty() {
        let numbers = Set::<u8>::new();

        assert_eq!(numbers.len(), 0);
    }

    #[test]
    fn set_lenght_increasing_when_inserted() {
        let mut numbers = Set::<u8>::new();

        assert_eq!(numbers.len(), 0);
        numbers.insert(1u8);
        assert_eq!(numbers.len(), 1);
    }

    #[test]
    fn set_lenght_not_increasing_when_reinserted() {
        let mut numbers = Set::<u8>::new();

        numbers.insert(1u8);
        assert_eq!(numbers.len(), 1);

        numbers.insert(1u8);
        assert_eq!(numbers.len(), 1);
    }

    #[test]
    fn set_lenght_increasing_when_different_elements_inserted() {
        let mut numbers = Set::<u8>::new();

        numbers.insert(1u8);
        assert_eq!(numbers.len(), 1);

        numbers.insert(2u8);
        assert_eq!(numbers.len(), 2);
    }
}
