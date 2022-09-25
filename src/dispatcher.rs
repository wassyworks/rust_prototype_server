use crate::packet_tag::PacketTag;
use std::collections::HashMap;
#[path = "socket.rs"]
mod socket;
use std::sync::Arc;

pub struct Dispatcher {
    tag_callback_map: HashMap<u16, Arc<dyn FnMut(&[u8])>>,
}

// NOTE: メンバのクロージャをスレッドセーフに扱う方法が不明のためコンパイルが通らない
impl Dispatcher {
    pub fn new() -> Dispatcher {
        Dispatcher {
            tag_callback_map: HashMap::<u16, Arc<dyn FnMut(&[u8])>>::new(),
        }
    }

    pub fn add(&mut self, tag: PacketTag, receiver: Arc<dyn FnMut(&[u8])>) {
        self.tag_callback_map.insert(tag as u16, receiver);
    }
    pub fn call(&self, tag: PacketTag, data: &[u8]) {
        match self.tag_callback_map.get(&(tag as u16)) {
            Some(func) => {
                func(data);
            }
            None => {
                println!("unkown tag. {:?}", tag);
            }
        }
    }
}
