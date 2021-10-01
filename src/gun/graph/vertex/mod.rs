use core::fmt;
use serde::Serialize;
use std::hash::Hash;

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
