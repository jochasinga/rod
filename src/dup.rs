use std::collections::HashMap;
use std::ops::Deref;
use crate::message::Message;

// NOTE:
// Object is a tuple struct with the inner `HashMap` as the only field.
// We provide a `Message` trait so the inner `HashMap` can be abstracted
// over and stay private to the caller.
// This leaves room for other implementation that might derive `Message`
// trait in the future, making it more flexible than a bare `HashMap`.
#[derive(Debug, PartialEq, Clone)]
pub struct Object<'a>(HashMap<&'a str, Value<'a>>);
impl<'a> Message<'a> for Object<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&'a mut self, key: &'a str, val: Value<'a>) -> &mut Self {
        let Self(h) = self;
        h.insert(key, val);
        self
    }
    fn get(&self, key: &str) -> Option<&Value> {
        let Self(h) = self;
        h.get(key)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Null,
    Bit(bool),
    Number(f32),
    Text(&'a str),
    Link(Object<'a>),
}

// Implement DUP as a unit struct for now, since it doesn't carry any properties.
pub struct Dup;

impl Dup {
    pub fn add<'a>(obj: &'a mut Object<'a>, key: &'a str, value: Value<'a>) -> Result<&'a mut Object<'a>, &'a str> {
        let obj = obj.insert(key, value);
        Ok(obj)
    }

    pub fn check<'a>(obj: &'a mut Object<'a>, id: &'a str) {
        unimplemented!("dup::check")
    }

    pub fn track<'a>(obj: &'a mut Object<'a>, id: &'a str) {
        unimplemented!("dup::track")
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dup() {
        let mut obj = Object::new();

        let result = Dup::add(&mut obj, "foo", Value::Null);

        if let Ok(_obj) = result {
            let val = _obj.get("foo").unwrap();
            assert_eq!(val, &Value::Null);
        }
    }
}