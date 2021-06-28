pub mod dup;

use std::time::{SystemTime};
use std::collections::HashMap;

// We pretty much need serde for JSON-related business.
// Unless we implement our own.
use serde::{self, Deserialize, Serialize};
use serde_json;

// Check out https://docs.rs/serde_json/1.0.64/serde_json/index.html#operating-on-untyped-json-values
// JSON representation as an enum serde_json::Value;
// We create this unit struct so it is analogous to JSON.parse in Javascript.
struct Json;
impl Json {
    // Convert raw JSON string slice to a type T.
    // This 'a is call a lifetime symbol. It is a way to tell the compiler
    // to reason about our code. Here, it simply says, data must at least
    // live as long as 'a, which is the lifetime (scope) of function parse.
    // Else, parse won't even consider taking it.
    // 
    // Type &str is a string slice, and is always in a borrowed form.
    // https://doc.rust-lang.org/stable/std/primitive.str.html
    fn parse<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T, String> {
        match serde_json::from_str(data) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("{}", e))
        }
    }
}

// So we derive special traits from serde crate. This basically makes
// a target struct JSON-serializable and -derializable.
#[derive(Serialize, Deserialize)]
struct Message {
    // Special macro magic from serde that
    // 1. rename the field name to and from JSON
    // 2. omit the field entirely if it is Option::None.
    #[serde(
        rename(deserialize = "#", serialize = "#"),
        skip_serializing_if = "Option::is_none"
    )]
    uuid: Option<String>,
    #[serde(
        rename(deserialize = "@", serialize = "@"),
        skip_serializing_if = "Option::is_none"
    )]
    ack: Option<String>,
}

type GenericMap<T> = HashMap<String, T>;

#[derive(Default)]
struct Dup<T> {
    // Correct me here!
    map: GenericMap<T>,
}

impl Dup<u128> {
    fn check(&self, id: &String) -> bool {
        // Well, actuallyyyyy, all you needed was this line :)
        // Option<...>.is_some() ask if it is Some variant and return a boolean.
        self.map.get(id).is_some()
    }

    // This method needs a mutable borrow of itself, since we are calling
    // insert() on the inner map.
    fn track(&mut self, id: &String) -> Result<u128, &str> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
          Ok(n) => {
              let ms = n.as_millis();
              self.map.insert(id.to_owned(), ms);
              return Ok(ms)
          }
          Err(_) => Err("SystemTime before UNIX EPOCH!"),
        }
    }
}

struct Dam<'a> {
    // This "version" of Dup carries a GenericMap<String, u128>,
    // assuming we want to store milliseconds value as u128.
    dup: &'a mut Dup<u128>,
}

impl Dam<'_> {
    //fn hear(dup: &Dup, raw: String) -> Option<()> {
    fn hear(&mut self, raw: &str) {

        // Yeah, you asked about a more succinct way, and I didn't think of this.
        // This is hard-core pattern-matching struct, ES6-style.

        // We call Json::parse::<TypeWeWantToDeserializeTo>::(raw_string)
        // as define above and just pattern-match with the left-hand side,
        // which is a type Result::Ok wrapping a Message struct.

        match Json::parse::<Message>(raw) {
            // How we dissect uuid out of our Message!
            Ok(Message{ uuid: Some(uuid), .. }) => {
                if self.dup.check(&uuid) {
                    println!("Yay");
                }

                if let Ok(v) = self.dup.track(&uuid) {
                    println!("{} tracked", v); 
                }
            }
            Ok(Message{ uuid: None, ..}) => {
                println!("UUID is None");
            }
            Err(err_msg) => {
                eprintln!("failed with error: {}", err_msg);
            }
        }
    }
}

fn main() {
    // Create a mutable borrow of a Dup struct, and save that
    // as a field in Dam.
    let dup = &mut Dup::default();
    let mut dam = Dam { dup };

    // now let's fake pass DAM data.
    let msg1 = "
    {
        \"#\": \"asdf\"
    }";


    dam.hear(msg1);

    let msg2 = "
    {
        \"@\": \"fghi\"
    }";

    dam.hear(msg2);
}


/*
struct Message {
    #: String,
    @: Option<String>,
    get: Option<?lex>,
    put: Option<?graph>,
    result: Result<Ack, MessageError>,
    err: Option<Number || String || object>,
    ack: Option<Number || string || lex>,
    ##: Option<String>
}
*/




