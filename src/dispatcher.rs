use crate::packet_tag::PacketTag;
use std::collections::HashMap;
#[path = "socket.rs"]
mod socket;

pub struct Dispatcher {
    // tag_callback_map: HashMap<PacketTag, Box<dyn PacketReceiver>>,
    tag_callback_map: HashMap<PacketTag, fn(&[u8])>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            tag_callback_map: HashMap::<PacketTag, fn(&[u8])>::new(),
        }
    }

    pub fn add(&mut self, tag: PacketTag, receiver: fn(&[u8])) {
        self.tag_callback_map.insert(tag, receiver);
    }
    pub fn call(&self, tag: PacketTag, data: &[u8]) {
        match self.tag_callback_map.get(&tag) {
            Some(func) => {
                func(data);
            }
            None => {
                println!("unkown tag. {:?}", tag);
            }
        }
    }
    // 上手くいかないので封印
    // pub fn Add<'de, T: PacketBase + Deserialize<'de>>(&mut self, func: fn(T)) {
    //     self.tag_callback_map.insert(
    //         PacketTag::SimpleEntity,
    //         Box::new(|bytes: &[u8]| match socket::deserialize::<'de, T>(bytes) {
    //             Some(pack) => {
    //                 func(pack);
    //             }
    //             None => {}
    //         }),
    //     );
    // }
}
