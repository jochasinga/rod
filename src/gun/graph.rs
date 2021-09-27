/// Definition:
/// A graph is an object consisting of two sets, a VertexSet<T> and an EdgeSet<T>.
/// The edge set may be empty, but otherwise its elements are two-element
/// subsets of the vertex set.

use std::collections::HashSet;
use std::hash::Hash;
use generational_arena::Arena;

pub type VertexSet<T> = Set<Vertex<T>>;
pub type EdgeSet<T> = Arena<Edge<T>>;

struct Graph<T> {
    vertices: VertexSet<T>,
    edges: EdgeSet<T>,
}

struct GraphBuilder<T> {
    vertices: VertexSet<T>,
    edges: EdgeSet<T>,
}

impl<T> GraphBuilder<T> 
where 
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        GraphBuilder {
            vertices: Set::new(),
            edges: Arena::new(),
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

    fn build(self) -> Result<Graph<T>, String> {
        let vertices = self.vertices.clone();
        let edges = self.edges.clone();
        if check_graph(self.vertices, self.edges) {
            Ok(Graph { vertices, edges })
        } else {
            Err("Not a graph".to_string())
        }
    }
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Graph {
            vertices: Set::new(),
            edges: Arena::new(),
        }
    }
}

// A vertex A is a finite non-empty set
// {A, B, C, D}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Vertex<T>(pub T);

impl<T> Vertex<T> 
where 
    T: Eq + Hash + Clone,
{
    fn new(data: T) -> Self {
        Vertex(data)
    }
}

#[derive(Clone)]
pub struct Set<T>(pub HashSet<T>);

impl<T> Set<T>
where 
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Set(HashSet::new())
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
}

impl<T> From<&Edge<T>> for Set<T>
where
    T: Eq + Hash + Clone,
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

impl<T> From<VertexSet<T>> for Set<T>
where 
    T: Eq + Hash + Clone,
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
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge<T>(pub Vertex<T>, pub Vertex<T>);

impl<T> Edge<T> 
where 
    T: Eq + Hash + Clone,
{
    fn new(v1: Vertex<T>, v2: Vertex<T>) -> Self {
        Edge(v1, v2)
    }
}

fn check_graph<T: Eq + Hash + Clone>(vertices: VertexSet<T>, edges: EdgeSet<T>) -> bool {

    if vertices.len() == 0 {
        return false;
    }

    for (_, e) in edges.iter() {
        let vertices_ = vertices.clone();
        let s: Set<T> = e.into();
        if s.len() > 2 {
            return false;
        }
        if !s.is_subset(&vertices_.into()) {
            return false;
        }
    }
    true
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

        Ok(())
    }
}

