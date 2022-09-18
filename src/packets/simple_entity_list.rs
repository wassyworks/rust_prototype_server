use serde::{Deserialize, Serialize};

use crate::packet_tag::PacketTag;
use crate::simple_entity::SimpleEntity;

// シリアライズテスト用構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleEntityList {
    simple_entity_list : Vec<SimpleEntity>
}

impl SimpleEntityList {
    pub fn new(simple_entity_list : Vec<SimpleEntity>) -> Self {
        SimpleEntityList {
            simple_entity_list 
        }
    }
}

#[allow(dead_code)]
pub fn get_tag() -> PacketTag {
    PacketTag::SimpleEntityList
}
