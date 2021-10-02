use core::fmt;
use serde::Serialize;
use std::hash::Hash;

mod builder;
mod edge;
mod set;
mod vertex;
use edge::Edge;

use builder::GraphBuilder;
use set::Set;
use std::cmp::PartialEq;
use std::iter::FromIterator;
use vertex::Vertex;
/// Definition:
/// A graph is an object consisting of two sets, a VertexSet<T> and an EdgeSet<T>.
/// The edge set may be empty, but otherwise its elements are two-element
/// subsets of the vertex set.

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

// #[cfg(test)]
// pub mod tests {

//     use super::*;

//     #[test]
//     fn test_graph() -> Result<(), String> {
//         let v1 = Vertex::new(33);
//         let v2 = Vertex::new(22);
//         let v3 = Vertex::new(11);
//         let v1_ = v1.clone();
//         let v1__ = v1.clone();
//         let v2_ = v2.clone();
//         let v2__ = v2.clone();
//         let v3_ = v3.clone();
//         let v3__ = v3.clone();

//         let v4 = Vertex::new(44);
//         let v5 = Vertex::new(55);
//         let v6 = Vertex::new(55);

//         let e1 = Edge::new(v1, v2);
//         let e2 = Edge::new(v2_, v3);
//         let e3 = Edge::new(v3_, v1_);

//         let g = GraphBuilder::<i32>::new()
//             .add_vertexset(VertexSet::<i32>::from_iter(
//                 vec![v1__, v2__, v3__, v4, v5, v6].iter(),
//             ))
//             .add_edgeset(EdgeSet::from_iter(vec![e1, e2, e3].iter()))
//             .build()?;

//         println!("{:#?}", g);
//         Ok(())
//     }
// }
