use std::collections::HashMap;
use std::convert::TryInto;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Object(String, HashMap<String, Value>);

impl Object {
    /// Create an Object with a random 32-bit id string.
    pub fn new() -> Self {
        Self(gen_random(32), HashMap::new())
    }

    /// Get the object's id.
    pub fn get_id(&self) -> String {
        self.0.to_owned()
    }

    /// Get the object's value.
    pub fn get(&self, key: String) -> Option<&Value> {
        self.1.get(&key)
    }

    /// Insert a Value into the object.
    pub fn insert(&mut self, key: String, val: Value) {
        self.1.insert(key, val);
    }
}

/// Build an Object according to the Builder pattern
/// Example:
/// 
/// ```rust
/// 
/// use rod::obj::ObjectBuilder;
/// let obj = ObjectBuilder::new()
///     .with_id("foobar")
///     .create();
/// assert!(obj.get_id() == "foobar".to_string());
/// 
/// let obj2 = ObjectBuilder::new().create();
/// assert!(obj2.get_id().len() == 32);
/// ```
pub struct ObjectBuilder {
    id: Option<String>,
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { id: None }
    }
    pub fn with_id(&mut self, id: &str) -> &mut ObjectBuilder {
        self.id = Some(id.to_string());
        self
    }
    pub fn create(&self) -> Object {
        if let Some(id) = &self.id {
            Object(id.to_string(), HashMap::new())
        } else {
            Object(gen_random(32), HashMap::new())
        }
    }
}

fn gen_random(len: i32) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bit(bool),
    Number(f32),
    Text(String),
    Link(Object),
}

impl TryInto<String> for &Value {
    type Error = &'static str;
    fn try_into(self) -> Result<String, Self::Error> {
        if let Value::Text(s) = self {
            Ok(s.to_string())
        } else {
            Err("Failed to convert to &str")
        }
    }
}