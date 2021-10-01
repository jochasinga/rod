use bincode;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::slice::Iter;
/// Definition:
/// A graph is an object consisting of two sets, a VertexSet<T> and an EdgeSet<T>.
/// The edge set may be empty, but otherwise its elements are two-element
/// subsets of the vertex set.
use std::{cmp::PartialEq, iter::FromIterator};

pub type VertexSet<T> = Set<Vertex<T>>;
pub type EdgeSet<T> = Set<Edge<T>>;

trait Isomorphic: PartialEq + Eq {
    fn is(&self, other: impl Isomorphic) -> bool;
}

#[derive(Debug, Serialize, Clone, Hash)]
pub struct Graph<T: Eq + Hash + Clone + Serialize + fmt::Debug> {
    vertices: VertexSet<T>,
    edges: EdgeSet<T>,
}

impl<T> PartialEq for Graph<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices && self.edges == other.edges
    }
}

impl<T: Eq + Hash + Clone + Serialize + fmt::Debug> Eq for Graph<T> {}

impl<T> Default for Graph<T>
where
    T: Default + Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn default() -> Self {
        Graph {
            vertices: Set::new(),
            edges: Set::new(),
        }
    }
}

struct GraphBuilder<T: Eq + Hash + Clone + Serialize + fmt::Debug> {
    vertices: VertexSet<T>,
    edges: EdgeSet<T>,
}

impl<T> Default for GraphBuilder<T>
where
    T: Default + Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn default() -> Self {
        GraphBuilder {
            vertices: Set::new(),
            edges: Set::new(),
        }
    }
}

impl<T> GraphBuilder<T>
where
    T: Eq + Hash + Clone + Serialize + core::fmt::Debug,
{
    pub fn new() -> Self {
        GraphBuilder {
            vertices: Set::new(),
            edges: Set::new(),
        }
    }

    fn add_vertex(mut self, v: Vertex<T>) -> Self {
        self.vertices.insert(v);
        self
    }

    fn add_edge(mut self, e: Edge<T>) -> Self {
        self.edges.insert(e);
        self
    }

    fn add_vertexset(mut self, vs: VertexSet<T>) -> Self {
        vs.into_iter().for_each(|v| {
            let v_ = v.clone();
            self.vertices.insert(v_);
        });
        self
    }

    fn add_edgeset(mut self, es: EdgeSet<T>) -> Self {
        es.into_iter().for_each(|e| {
            let e_ = e.clone();
            self.edges.insert(e_);
        });
        self
    }

    fn link_adjacent_vertices(mut self) -> Result<Self, String> {
        // Build a subgraph for every vertex and save it.

        let mut vs = Set::<Vertex<T>>::new();
        let mut es = Set::<Edge<T>>::new();

        for v in self.vertices.into_iter() {
            for e in self.edges.into_iter() {
                if e.is_incident(&v) {
                    vs.insert(v.clone());
                    es.insert(e.clone());
                }
            }
        }

        Ok(self)
    }

    fn build(self) -> Result<Graph<T>, String> {
        let vertices = self.vertices.clone();
        let edges = self.edges.clone();
        if Graph::is_graph(self.vertices, self.edges) {
            Ok(Graph { vertices, edges })
        } else {
            Err("Not a graph".to_string())
        }
    }
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn new() -> Self {
        Graph {
            vertices: Set::new(),
            edges: Set::new(),
        }
    }

    fn is_graph(vertices: VertexSet<T>, edges: EdgeSet<T>) -> bool {
        if vertices.len() == 0 {
            return false;
        }

        for e in edges.iter() {
            let vertices_ = vertices.clone();
            let es: Set<T> = e.into();
            if es.len() != 2 {
                return false;
            }
            if !es.is_subset(&vertices_.into()) {
                return false;
            }
        }
        true
    }
}

// A vertex set V is a finite non-empty set
// V = {A, B, C, D}
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

impl<T> Eq for Vertex<T> where T: Eq + Hash + Clone + Serialize + fmt::Debug {}

impl<T> Vertex<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn new(data: T) -> Self {
        Vertex(data)
    }

    fn is_incident(&self, e: &Edge<T>) -> bool {
        let vs: Set<T> = self.into();
        let es: Set<T> = e.into();
        es.intersection(&vs) == vs
    }

    /// Returns a Set<Vertex<T>> of all adjacent vertices and edges
    fn adjc(&self) -> Option<&Graph<T>> {
        todo!("Implement adj() that returns a sub graph with only adjacent vertices");
    }
}

/// Set<T> is the base set wrapping the HashSet primitive and
/// providing certain set abstractions.
/// WARNING: Casting Vertex<T> or Edge<T> to Set<T> are lossy
/// meaning once the casting is done there is no way of differentiating
/// a Set<T> if it was previously of type Vertex<T> or Edge<T>
/// aside from its size. Therefore, this casting should be limited
/// to just within the implementation of methods of Vertex<T> and Edge<T> only.
///
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
        h1.difference(&h2).count() == 0
    }
}

impl<T> Eq for Set<T> where T: Eq + Hash + Clone + fmt::Debug {}

impl<T> Set<T>
where
    T: Eq + Hash + Clone + fmt::Debug,
{
    fn new() -> Self {
        Set(HashSet::new())
    }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, T> {
        let Set(h) = self;
        h.iter()
    }

    fn insert(&mut self, data: T) {
        let Set(inner) = self;
        inner.insert(data);
        let new_inner = inner.clone();
        *self = Set(new_inner);
    }

    fn len(&self) -> usize {
        let Set(hs) = self;
        hs.len()
    }

    fn is_subset(&self, other: &Set<T>) -> bool {
        let (Set(h1), Set(h2)) = (self, other);
        h1.is_subset(h2)
    }

    fn intersection(&self, other: &Self) -> Self {
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
        hs.into_iter()
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
                Some(d) => Ok(Vertex(d.clone())),
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

/// An edge connects {A, B} where A and B are elements of a set of vertices V.
/// {A, B} ⊆ V where A ∈ V & B ∈ V
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default, Serialize)]
pub struct Edge<T: Eq + Hash + Clone + Serialize + fmt::Debug>(pub Vertex<T>, pub Vertex<T>);

impl<T> Edge<T>
where
    T: Eq + Hash + Clone + Serialize + fmt::Debug,
{
    fn new(v1: Vertex<T>, v2: Vertex<T>) -> Self {
        Edge(v1, v2)
    }

    fn is_incident(&self, v: &Vertex<T>) -> bool {
        // let Edge(v1, v2) = self;
        let es: Set<T> = self.into();
        let vs: Set<T> = v.into();
        es.intersection(&vs) == vs
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_graph() -> Result<(), String> {
        let v1 = Vertex::new(33);
        let v2 = Vertex::new(22);
        let v3 = Vertex::new(11);
        let v1_ = v1.clone();
        let v1__ = v1.clone();
        let v2_ = v2.clone();
        let v2__ = v2.clone();
        let v3_ = v3.clone();
        let v3__ = v3.clone();

        let v4 = Vertex::new(44);
        let v5 = Vertex::new(55);
        let v6 = Vertex::new(55);

        let e1 = Edge::new(v1, v2);
        let e2 = Edge::new(v2_, v3);
        let e3 = Edge::new(v3_, v1_);

        let g = GraphBuilder::<i32>::new()
            .add_vertexset(Set::<Vertex<i32>>::from_iter(
                vec![v1__, v2__, v3__, v4, v5, v6].iter(),
            ))
            .add_edgeset(EdgeSet::from_iter(vec![e1, e2, e3].iter()))
            .build()?;

        println!("{:#?}", g);
        Ok(())
    }

    /*
    #[test]
    fn test_graph_get() -> Result<(), String> {
        let v1 = Vertex::new(33);
        let v2 = Vertex::new(22);
        let v3 = Vertex::new(11);
        let v1_ = v1.clone();
        let v1__ = v1.clone();
        let v2_ = v2.clone();
        let v2__ = v2.clone();
        let v3_ = v3.clone();
        let v3__ = v3.clone();

        let e1 = Edge::new(v1, v2);
        let e2 = Edge::new(v2_, v3);
        let e3 = Edge::new(v3_, v1_);

        let graph = GraphBuilder::<i32>::new()
            .add_vertex(v1__)
            .add_vertex(v2__)
            .add_vertex(v3__)
            .add_edge(e1)
            .add_edge(e2)
            .add_edge(e3)
            .build()?;

        let vertex = graph.get(33).unwrap();
        assert_eq!(vertex, Vertex(33));

        Ok(())
    }
    */
}
