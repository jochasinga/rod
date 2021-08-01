use crate::dup::Value;
use std::str::FromStr;

/// Key represents constraint in the Message's keys.
/// This makes it safe to use Message's method.
pub enum Key {
    MessageId,
    ContentHash,
    AckId,
    Peers,
    Via,
}

// impl Key {
//     fn as_str(&self) -> &str {
//         match self {
//             Self::MessageId => "#",
//             Self::ContentHash => "##",
//             Self::AckId => "@",
//             Self::Peers => "><",
//         }
//     }
// }

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Self::MessageId   => String::from("#"),
            Self::ContentHash => String::from("##"),
            Self::AckId       => String::from("@"),
            Self::Peers       => String::from("><"),
            Self::Via         => String::from("via"),
        }
    }
}

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#"   => Ok(Key::MessageId),
            "##"  => Ok(Key::ContentHash),
            "@"   => Ok(Key::AckId),
            "><"  => Ok(Key::Peers),
            "via" => Ok(Key::Via),
            _     => Err(format!("{} is not a valid key", s)),
        }
    }
}

pub trait Message {
    fn insert(&mut self, key: Key, val: Value) -> Result<(), &str>;
    fn get(&self, key: Key) -> Option<Value>;
}