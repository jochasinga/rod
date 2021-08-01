use std::convert::TryInto;

use crate::dup::{Dup, Object, Value};
use crate::gun::Gun;
use crate::message::{Message, Key};


struct Dam;
impl Dam {
    fn hear(mut msg: Box<dyn Message>, _peer: &mut Object, gun: &Gun) -> Result<(), String> {
        if let Some(id_val) = msg.get(Key::MessageId) {
            if let Ok(id_str) = id_val.clone().try_into() {
                if Dup::check(&gun.dups, id_str) {
                    return Ok(());
                }
            }
        }

        // FIXME: Review this logic!
        if let (Some(ack), Some(hash)) = 
            (msg.get(Key::AckId), msg.get(Key::ContentHash))
            {
                match (ack, hash) {
                (Value::Link(obj), Value::Text(hash_str)) => {
                    if Dup::check(&obj, hash_str) {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        let id =  msg.get(Key::MessageId);
        let _id = id.clone();

        if let Some(id_val) = _id {
            msg.insert(Key::Via, id_val)?;

            if let Some(v) = id {
                match v {
                    Value::Text(id_str) => {
                        Dup::track(&gun.dups, id_str);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        // peers already set to, we will not want to relay again to them.
        // We're not using this?
        let _near = msg.get(Key::Peers);


        gun.inbound(&msg);


        if let Some(id_val) = msg.get(Key::MessageId) {
            if let Ok(id_str) = id_val.try_into() {
                Dup::track(&gun.dups, id_str);

                // Where did `ash` come from?
                // Dup::track(&gun.dups, ash);
            }
        }

        Ok(())
    }

    // One can only borrow as mutable once, so not sure if this is legit or necessary.
    fn hear_one(_msg: Box<dyn Message>, _peer: &mut Object) {
        unimplemented!("Dam::hear");
    }

    fn say(_msg: &mut Object, _peer: &mut Object) {
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