use crate::dup::Object;

struct Dam;
impl Dam {
    fn hear(msg: &str, peer: &mut Object) {
        unimplemented!("Dam::hear");
    }

    // One can only borrow as mutable once, so not sure if this is legit or necessary.
    fn hear_one(msg: &mut Object, peer: &mut Object) {
        unimplemented!("Dam::hear");
    }

    fn say(msg: &mut Object, peer: &mut Object) {
        unimplemented!("Dam::say");
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_dam() {
        assert_eq!(true, false);
    }
}