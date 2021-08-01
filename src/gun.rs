use crate::dup::Object;
use crate::message::Message;

pub struct Gun<'a> {
    pub dups: &'a Object
}

impl<'a> Gun<'_> {
    // One can only borrow as mutable once, so not sure if this is legit or necessary.
    pub fn get(_msg: Box<dyn Message>, _peer: &mut Object) {
        unimplemented!("Gun::get");
    }

    // TODO: "in" is a reserve word in Rust.
    pub fn inbound(&self, _msg: &Box<dyn Message>) {
        unimplemented!("Gun.in");
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_dam() {
        assert_eq!(true, false);
    }
}