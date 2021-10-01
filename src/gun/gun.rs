use crate::obj::Object;

pub struct Gun<'a> {
    pub dups: &'a Object,
}

impl<'a> Gun<'_> {
    // One can only borrow as mutable once, so not sure if this is legit or necessary.
    pub fn get(_msg: Object, _peer: &mut Object) {
        unimplemented!("Gun::get");
    }

    // TODO: "in" is a reserve word in Rust.
    pub fn inbound(&self, _msg: &Object) {
        unimplemented!("Gun.in");
    }
}
