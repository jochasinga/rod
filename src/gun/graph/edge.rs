use core::fmt;
use serde::Serialize;
use std::collections::HashSet;
use std::hash::Hash;
use std::slice::Iter;
use std::{cmp::PartialEq, iter::FromIterator};

use super::{Set, Vertex, VertexSet};

/// An edge connects {A, B} where A and B are elements of a set of vertices V.
/// {A, B} ⊆ V where A ∈ V & B ∈ V
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default, Serialize)]
pub struct Edge<T: Eq + Hash + Clone + Serialize + fmt::Debug>(pub Vertex<T>, pub Vertex<T>);

impl<T> Edge<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    pub fn new(v1: Vertex<T>, v2: Vertex<T>) -> Self {
        Self(v1, v2)
    }

    pub fn is_incident(&self, v: &Vertex<T>) -> bool {
        let Edge(v1, v2) = self;
        v1.eq(&v) || v2.eq(&v)
    }
}

impl<'a, T> FromIterator<&'a Edge<T>> for Set<Edge<T>>
where
    T: 'a + Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = &'a Edge<T>>>(iter: I) -> Self {
        let mut hs = HashSet::new();
        for i in iter {
            let i_ = i.clone();
            hs.insert(i_);
        }
        Set(hs)
    }
}

impl<T> From<Iter<'_, Edge<T>>> for Set<Edge<T>>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(it: Iter<Edge<T>>) -> Self {
        let mut hs = HashSet::<Edge<T>>::new();
        it.for_each(|e| {
            let e_ = e.clone();
            hs.insert(e_);
        });
        Set::from(hs)
    }
}

impl<T> From<&Edge<T>> for Set<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(e: &Edge<T>) -> Self {
        let Edge(v1, v2) = e;
        let mut hs = HashSet::<Vertex<T>>::new();
        hs.insert(v1.clone());
        hs.insert(v2.clone());
        let vs: VertexSet<T> = Set(hs);
        vs.into()
    }
}

impl<T> From<&Edge<T>> for Set<Edge<T>>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(e: &Edge<T>) -> Self {
        let mut hs = HashSet::<Edge<T>>::new();
        let e_ = e.clone();
        hs.insert(e_);
        Set(hs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn edge_can_be_created_with_vertices() {
        let from = Vertex::new(1);
        let to = Vertex::new(1);
        let from_clone = from.clone();
        let to_clone = to.clone();

        let example_edge = Edge::new(from, to);
        assert_eq!(example_edge.0, from_clone);
        assert_eq!(example_edge.1, to_clone);
    }

    #[test]
    fn edge_can_assume_incident_verices() {
        let from = Vertex::new(1);
        let to = Vertex::new(1);
        let from_clone = from.clone();
        let to_clone = to.clone();
        let another_vertex = Vertex::new(100);

        let example_edge = Edge::new(from, to);
        assert_eq!(example_edge.is_incident(&from_clone), true);
        assert_eq!(example_edge.is_incident(&to_clone), true);
        assert_eq!(example_edge.is_incident(&another_vertex), false);
    }
}
