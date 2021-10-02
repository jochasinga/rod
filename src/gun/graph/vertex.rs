use core::fmt;
use serde::Serialize;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::Hash;
use std::{cmp::PartialEq, iter::FromIterator};

use super::set::Set;
use super::Edge;
use super::Graph;
use super::VertexSet;

#[derive(Hash, Clone, Debug, Serialize, Default)]
pub struct Vertex<T: Eq + Hash + Clone + Serialize + fmt::Debug>(pub T);

impl<T> PartialEq for Vertex<T>
where
    T: Eq + PartialEq + Hash + Clone + Serialize + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> TryFrom<Set<T>> for Vertex<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    type Error = String;

    fn try_from(s: Set<T>) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            Err("Set<T> must have size of 1 or less".to_string())
        } else {
            let mut iter = s.0.into_iter().take(1);
            match iter.next() {
                Some(d) => Ok(Vertex(d)),
                _ => Err("End of iter".to_string()),
            }
        }
    }
}

impl<T> From<&Vertex<T>> for Set<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(v: &Vertex<T>) -> Self {
        let mut hs = HashSet::<T>::new();
        let Vertex(d) = v.clone();
        hs.insert(d);
        Set(hs)
    }
}

impl<T> From<&Vertex<T>> for Set<Vertex<T>>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(v: &Vertex<T>) -> Self {
        let mut hs = HashSet::<Vertex<T>>::new();
        let v_ = v.clone();
        hs.insert(v_);
        Set(hs)
    }
}

impl<T> From<VertexSet<T>> for Set<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from(vst: VertexSet<T>) -> Self {
        let Set(vs) = vst;
        let mut hs = HashSet::<T>::new();
        for v in vs.iter() {
            let Vertex(t) = v;
            let t_ = t.clone();
            hs.insert(t_);
        }
        Set(hs)
    }
}

impl<T> Eq for Vertex<T> where T: Eq + Hash + Clone + Serialize + fmt::Debug {}

impl<T> Vertex<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    pub fn new(data: T) -> Self {
        Vertex(data)
    }

    pub fn is_incident(&self, e: &Edge<T>) -> bool {
        let vs: Set<T> = self.into();
        let es: Set<T> = e.into();
        es.intersection(&vs) == vs
    }

    /// Returns a Set<Vertex<T>> of all adjacent vertices and edges
    fn adjc(&self) -> Option<&Graph<T>> {
        todo!("Implement adj() that returns a sub graph with only adjacent vertices");
    }
}

impl<'a, T> FromIterator<&'a Vertex<T>> for Set<Vertex<T>>
where
    T: 'a + Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn from_iter<I: IntoIterator<Item = &'a Vertex<T>>>(iter: I) -> Self {
        let mut hs = HashSet::new();
        for i in iter {
            let i_ = i.clone();
            hs.insert(i_);
        }
        Set(hs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vertex_can_be_created_with_constructor() {
        let example_vertex = Vertex::<u8>::new(1);
        assert_eq!(example_vertex.0, 1);
    }

    #[test]
    fn vertex_is_comperable_with_another_vertices() {
        let example_vertex = Vertex::<u8>::new(1);
        let another_with_same_value = Vertex::<u8>::new(1);
        let another_with_different_value = Vertex::<u8>::new(2);

        assert_eq!(example_vertex.eq(&another_with_same_value), true);
        assert_eq!(example_vertex.eq(&another_with_different_value), false);
    }

    #[test]
    fn vertex_is_incident_on_edge() {
        let vertex_from = Vertex::<u8>::new(1);
        let vertex_to = Vertex::<u8>::new(2);
        let vertex_to_copy = vertex_to.clone();

        let vertex_from_copy = vertex_from.clone();

        let example_edge = Edge::new(vertex_from, vertex_to);

        assert_eq!(example_edge.is_incident(&vertex_from_copy), true);
        assert_eq!(example_edge.is_incident(&vertex_to_copy), true);
    }

    #[test]
    fn differnet_vertex_is_incident_on_edge_where_vertex_not_exists() {
        let vertex_from = Vertex::<u8>::new(1);
        let vertex_to = Vertex::<u8>::new(2);
        let different_vertex = Vertex::new(13);

        let example_edge = Edge::new(vertex_from, vertex_to);

        assert_eq!(example_edge.is_incident(&different_vertex), false);
    }
}
