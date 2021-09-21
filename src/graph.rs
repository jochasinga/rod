use std::collections::HashSet;
use std::convert::TryFrom;

/// Definition:
/// A graph is an object consisting of two sets, a vertex set and an edge set.
/// The edge set may be empty, but otherwise its elements are two-element
/// subsets of the vertex set.
trait Graph<T> : TryFrom<S> {
  fn is_isomorphic(&self, other: Box<dyn Graph>) -> bool;
}

impl TryFrom<S> for Graph<T> {
  type Error = &'static str;

  fn try_from(value: S) -> Result<Self, Self::Error> {
    todo!("Implement TryFrom<S> so it can convert an object to Graph<T>");
  }
}

/// A starting implementation of [graph::Graph].
struct NaiveGraph<T> {
  vertices: HashSet<Vertex<T>>,
  edges: HashSet<Edge<T>>,
}

impl Graph for NaiveGraph {
  fn is_isomorphic(&self, other: Box<dyn Graph>) -> bool {
    todo!("Implement Graph");
  }
}


// A vertex A is where 
#[derive(PartialEq, Eq, Hash)]
struct Vertex<T>(pub T);

// An edge connects (A, B) where A and B are vertices.
#[derive(PartialEq, Eq, Hash)]
struct Edge<T>(pub Vertex<T>, pub Vertex<T>);
