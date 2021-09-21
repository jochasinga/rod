pub mod gun;

// Public-export submodules
pub use crate::gun::*;

#[cfg(test)]
mod tests {

    use super::*;
    use obj::{Object, Value};
    use dup::Dup;
    use message::Key;

    #[test]
    fn test_dup() {
        let mut obj = Object::new();
        let key: String = Key::AckId.to_string();
        let _ = Dup::add(&mut obj, key.clone(), Value::Null);
        if let Some(val) = obj.get(key) {
            assert_eq!(*val, Value::Null);
        } else {
            assert!(false);
        }
    }
}
