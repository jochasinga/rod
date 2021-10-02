use core::fmt;
use serde::Serialize;
use std::hash::Hash;

use super::{Edge, EdgeSet, Graph, Set, Vertex, VertexSet};

pub struct GraphBuilder<T: Eq + Hash + Clone + Serialize + fmt::Debug> {
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
        Self {
            vertices: Set::new(),
            edges: Set::new(),
        }
    }

    pub fn add_vertex(mut self, v: Vertex<T>) -> Self {
        self.vertices.insert(v);
        self
    }

    pub fn add_edge(mut self, e: Edge<T>) -> Self {
        self.edges.insert(e);
        self
    }

    pub fn add_vertexset(mut self, vs: VertexSet<T>) -> Self {
        vs.into_iter().for_each(|v| {
            let v_ = v.clone();
            self.vertices.insert(v_);
        });
        self
    }

    pub fn add_edgeset(mut self, es: EdgeSet<T>) -> Self {
        es.into_iter().for_each(|e| {
            let e_ = e.clone();
            self.edges.insert(e_);
        });
        self
    }

    pub fn link_adjacent_vertices(self) -> Result<Self, String> {
        // Build a subgraph for every vertex and save it.

        let mut vs = Set::<Vertex<T>>::new();
        let mut es = Set::<Edge<T>>::new();

        for v in self.vertices.into_iter() {
            for e in self.edges.into_iter() {
                if e.is_incident(v) {
                    vs.insert(v.clone());
                    es.insert(e.clone());
                }
            }
        }

        Ok(self)
    }

    pub fn build(self) -> Result<Graph<T>, String> {
        let vertices = self.vertices.clone();
        let edges = self.edges.clone();
        if Graph::is_graph(self.vertices, self.edges) {
            Ok(Graph { vertices, edges })
        } else {
            Err("Not a graph".to_string())
        }
    }
}
