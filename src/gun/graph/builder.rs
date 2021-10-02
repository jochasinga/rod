use core::fmt;
use serde::Serialize;
use std::hash::Hash;
use std::iter::FromIterator;

use super::{Edge, EdgeSet, Graph, Set, Vertex, VertexSet};

#[derive(Debug, Clone)]
pub struct GraphError(pub String);

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

    pub fn link_adjacent_vertices(self) -> Result<Self, GraphError> {
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

    pub fn build(self) -> Result<Graph<T>, GraphError> {
        let vertices = self.vertices.clone();
        let edges = self.edges.clone();
        if Graph::is_graph(self.vertices, self.edges) {
            Ok(Graph { vertices, edges })
        } else {
            Err(GraphError("It is not a valid graph".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_empty_edgeset_with_default() {
        let graph = GraphBuilder::<u8>::default();
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn builder_has_edgeset_when_its_is_given() -> Result<(), GraphError> {
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

        GraphBuilder::<i32>::new()
            .add_vertexset(VertexSet::<i32>::from_iter(
                vec![v1__, v2__, v3__, v4, v5, v6].iter(),
            ))
            .add_edgeset(EdgeSet::from_iter(vec![e1, e2, e3].iter()))
            .build()?;

        Ok(())
    }
}
