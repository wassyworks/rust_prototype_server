pub struct SimpleEntityReceiver {
    packet: SimpleEntityList,
}
use crate::simple_entity_list::SimpleEntityList;
use crate::simple_entity_manager::SimpleEntityManager;
use std::sync::Arc;

impl SimpleEntityReceiver {
    pub fn new() -> SimpleEntityReceiver {
        SimpleEntityReceiver {
            packet: SimpleEntityList::new(Vec::new()),
        }
    }
}

pub fn make_receive_fn(manager: &mut SimpleEntityManager) -> Arc<dyn FnMut(&[u8])> {
    Arc::new(|bytes: &[u8]| {
        match bincode::deserialize::<SimpleEntityList>(bytes) {
            Ok(decoded) => {
                manager.add(decoded);
            }
            Err(e) => {
                println!("failed to deserialization. {}", e.to_string());
            }
        }
        println!("simple entity receiver: received");
    })
}
