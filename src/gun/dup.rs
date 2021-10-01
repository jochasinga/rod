use crate::obj::{Object, Value};

pub struct Dup;

impl Dup {
    pub fn add<'a>(obj: &'a mut Object, key: String, value: Value) {
        obj.insert(key, value)
    }

    pub fn check(_obj: &Object, _id: String) -> bool {
        unimplemented!("dup::check")
    }

    pub fn track(_obj: &Object, _id: String) {
        unimplemented!("dup::track")
    }
}
