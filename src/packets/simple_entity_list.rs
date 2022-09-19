use serde::{Deserialize, Serialize};

use crate::packet_base::PacketBase;
use crate::packet_tag::PacketTag;
use crate::simple_entity::SimpleEntity;

// シリアライズテスト用構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleEntityList {
    simple_entity_list: Vec<SimpleEntity>,
}

impl PacketBase for SimpleEntityList {
    #[allow(dead_code)]
    fn get_tag(&self) -> PacketTag {
        PacketTag::SimpleEntityList
    }
}

impl SimpleEntityList {
    pub fn new(simple_entity_list: Vec<SimpleEntity>) -> Self {
        SimpleEntityList { simple_entity_list }
    }
}
