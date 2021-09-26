/// Definition:
/// A graph is an object consisting of two sets, a VertexSet<T> and an EdgeSet<T>.
/// The edge set may be empty, but otherwise its elements are two-element
/// subsets of the vertex set.

use std::collections::HashSet;
use std::hash::Hash;
use generational_arena::Arena;

pub type VertexSet<T> = HashSet<T>;
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
            vertices: HashSet::new(),
            edges: Arena::new(),
        }
    }

    fn add_vertex(mut self, v: T) -> Self {
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
            vertices: HashSet::new(),
            edges: Arena::new(),
        }
    }
}

// A vertex A is a finite non-empty set
// {A, B, C, D}
// #[derive(PartialEq, Eq, Hash)]
pub type Vertex<T> = HashSet<T>;

/// An edge connects {A, B} where A and B are elements of a set of vertices V.
/// {A, B} ⊆ V where A ∈ V & B ∈ V
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge<T>(pub T, pub T);

impl<T> Into<HashSet<T>> for &Edge<T> 
where 
    T: Eq + Hash + Clone,
{
    fn into(self) -> HashSet<T> {
        let mut set = HashSet::new();
        set.insert(self.0.clone());
        set.insert(self.1.clone());
        set
    }
}

fn check_graph<T: Eq + Hash + Clone>(vertices: VertexSet<T>, edges: EdgeSet<T>) -> bool {

    if vertices.len() == 0 {
        return false;
    }

    for (_, e) in edges.iter() {
        let h: HashSet<T> = (e).into();
        if h.len() > 2 {
            return false;
        }
        if !h.is_subset(&vertices) {
            return false;
        }
    }
    true
}