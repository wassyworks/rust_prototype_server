use crate::simple_entity_list::SimpleEntityList;
use std::collections::HashMap;

pub struct SimpleEntityManager {
    packets: HashMap<u64, SimpleEntityList>,
}

impl SimpleEntityManager {
    pub fn new() -> SimpleEntityManager {
        SimpleEntityManager {
            packets: HashMap::new(),
        }
    }

    pub fn add(&mut self, list: SimpleEntityList) {
        self.packets.insert(1, list);
    }
}
