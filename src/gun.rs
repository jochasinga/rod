use crate::dup::Object;

struct Gun;
impl Gun {
    // One can only borrow as mutable once, so not sure if this is legit or necessary.
    fn get(msg: &mut Object, peer: &mut Object) {
        unimplemented!("Gun::get");
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_dam() {
        assert_eq!(true, false);
    }
}