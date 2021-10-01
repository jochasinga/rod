use crate::dup::Dup;
use crate::gun::gun::Gun;
use crate::message::{Key, Message};
use crate::obj::{Object, Value};
/// Currently not in use!
use std::convert::TryInto;

// Daisy-chain Ad-hoc Mesh-networking
struct Dam;
impl Dam {
    fn hear(msg: Object, _peer: &mut Object, gun: &Gun) -> Result<(), String> {
        if let Some(id_val) = msg.get(Key::MessageId.to_string()) {
            if let Ok(id_string) = id_val.try_into() {
                if Dup::check(gun.dups, id_string) {
                    return Ok(());
                }
            }
        }

        let ack_id = Key::AckId.to_string();
        let message_id = Key::MessageId.to_string();
        let content_id = Key::ContentHash.to_string();

        // FIXME: Review this logic!
        if let (Some(ack), Some(hash)) = (msg.get(ack_id), msg.get(content_id)) {
            match (ack, hash) {
                (Value::Link(obj), Value::Text(hash_str)) => {
                    if Dup::check(obj, hash_str.to_string()) {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        let v = msg.get(message_id.clone()).unwrap();
        match v {
            Value::Text(s) => {
                Dup::track(gun.dups, s.clone());
                return Ok(());
            }
            _ => {}
        }

        // peers already set to, we will not want to relay again to them.
        // We're not using this?
        let _near = msg.get(Key::Peers.to_string());

        gun.inbound(&msg);

        if let Some(id_val) = msg.get(message_id) {
            if let Ok(id_str) = id_val.try_into() {
                Dup::track(gun.dups, id_str);

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
