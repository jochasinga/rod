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

    #[test]
    fn is_subset_with_empty_sets() {
        let empty_first = Set::<u8>::new();
        let empty_second = Set::<u8>::new();
        assert_eq!(empty_first.is_subset(&empty_second), true);
    }

    #[test]
    fn is_subset_should_be_false_with_non_empty_is_subset_of_empty() {
        let mut non_empty = Set::<u8>::new();
        non_empty.insert(1u8);
        let empty = Set::<u8>::new();
        assert_eq!(non_empty.is_subset(&empty), false);
    }

    #[test]
    fn is_subset_should_be_true_with_empty_is_subset_of_non_empty() {
        let mut non_empty = Set::<u8>::new();
        non_empty.insert(1u8);
        let empty = Set::<u8>::new();
        assert_eq!(empty.is_subset(&non_empty), true);
    }

    #[test]
    fn intersection_of_two_empty_set_should_be_empty() {
        let empty_first = Set::<u8>::new();
        let empty_second = Set::<u8>::new();

        assert_eq!(empty_first.intersection(&empty_second).len(), 0);
    }

    #[test]
    fn intersection_of_one_non_empty_and_empty_set_should_be_the_non_empty_set() {
        let mut non_empty = Set::<u8>::new();
        non_empty.insert(2u8);
        non_empty.insert(1u8);
        let empty = Set::<u8>::new();

        assert_eq!(non_empty.intersection(&empty), non_empty);
    }

    #[test]
    fn intersection_of_two_none_empty_with_no_common_elements_should_be_the_intersection_of_the_two_set_count(
    ) {
        let mut non_empty_first = Set::<u8>::new();
        non_empty_first.insert(1u8);
        non_empty_first.insert(2u8);

        let mut non_empty_second = Set::<u8>::new();
        non_empty_second.insert(3u8);
        non_empty_second.insert(4u8);

        let mut expected_intersection = Set::<u8>::new();
        expected_intersection.insert(1u8);
        expected_intersection.insert(2u8);
        expected_intersection.insert(3u8);
        expected_intersection.insert(4u8);

        assert_eq!(
            non_empty_first.intersection(&non_empty_second),
            expected_intersection
        );
    }

    #[test]
    fn intersection_of_two_none_empty_with_common_elements_count_should_be_the_intersection_of_the_two_set_count(
    ) {
        let mut non_empty_first = Set::<u8>::new();
        non_empty_first.insert(1u8);
        non_empty_first.insert(2u8);

        let mut non_empty_second = Set::<u8>::new();
        non_empty_second.insert(1u8);
        non_empty_second.insert(4u8);

        let mut expected_intersection = Set::<u8>::new();
        expected_intersection.insert(1u8);
        expected_intersection.insert(2u8);
        expected_intersection.insert(4u8);

        assert_eq!(
            non_empty_first.intersection(&non_empty_second),
            expected_intersection
        );
    }
}
