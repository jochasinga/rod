use std::collections::HashMap;

type Object<'a> = HashMap<&'a str, Value<'a>>;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Null,
    Bit(bool),
    Number(f32),
    Text(&'a str),
    Link(Object<'a>),
}

// Implement DUP as a module for now, since it doesn't carry any properties.
pub mod dup {

    use super::*;

    pub fn add<'a>(obj: &'a mut Object<'a>, key: &'a str, value: Value<'a>) -> Result<&'a mut Object<'a>, ()> {
        obj.insert(key, value);
        Ok(obj)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dup() {
        let mut obj = HashMap::new();

        if let Ok(o) = dup::add(&mut obj, "foo", Value::Null) {
            let val = o.get("foo").unwrap();
            assert_eq!(*val, Value::Null);
        }
    }
}