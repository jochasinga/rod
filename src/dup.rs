use std::{collections::HashMap, convert::TryInto};
use crate::message::{Message, Key};

// NOTE:
// Object is a tuple struct with the inner `HashMap` as the only field.
// We provide a `Message` trait so the inner `HashMap` can be abstracted
// over and stay private to the caller.
// This leaves room for other implementation that might derive `Message`
// trait in the future, making it more flexible than a bare `HashMap`.
#[derive(Debug, PartialEq, Clone)]
pub struct Object(HashMap<String, Value>);

impl Object {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

// Object is a wrapper type over `HashMap<&'a str, Value<'a>>` that implements `Message`.
impl Message for Object {
    fn insert(&mut self, key: Key, val: Value) -> Result<(), &str> {
        let Self(h) = self;
        match h.insert(key.to_string(), val) {
            Some(_) => Ok(()),
            None => Err("Oh no"),
        }
    }

    fn get(&self, key: Key) -> Option<Value> {
        let Self(h) = self;
        match h.get(&key.to_string()) {
            Some(v) => Some(v.clone()),
            None          => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bit(bool),
    Number(f32),
    Text(String),
    Link(Object),
}

impl TryInto<String> for Value {
    type Error = &'static str;
    fn try_into(self) -> Result<String, Self::Error> {
        if let Value::Text(s) = self {
            Ok(s)
        } else {
            Err("Failed to convert to &str")
        }
    }
}

// Implement DUP as a unit struct for now, since it doesn't carry any properties.
pub struct Dup;

impl Dup {
    pub fn add<'a>(obj: &'a mut Object, key: Key, value: Value) -> Result<(), &'a str> {
        obj.insert(key, value)
    }

    pub fn check(_obj: &Object, _id: String) -> bool {
        unimplemented!("dup::check")
    }

    pub fn track(_obj: &Object, _id: String) {
        unimplemented!("dup::track")
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dup() {
        let mut obj = Object::new();
        let _ = Dup::add(&mut obj, Key::AckId, Value::Null).unwrap();
        let val = obj.get(Key::AckId).unwrap();
        assert_eq!(val, Value::Null);
    }
}