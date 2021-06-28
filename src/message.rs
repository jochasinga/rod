use crate::dup;

pub trait Message<'a> {
    fn new() -> Self;
    fn insert(&'a mut self, key: &'a str, val: dup::Value<'a>) -> &mut Self;
    fn get(&self, key: &str) -> Option<&dup::Value>;
}