use serde::{Deserialize, Serialize};

use crate::packet_tag::PacketTag;

// シリアライズテスト用構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleEntity {
    player_id: u64,
    x: f32,
    y: f32,
    name: String,
    hp: u32,
}

impl SimpleEntity {
    pub fn new(player_id: u64, x: f32, y: f32, name: String, hp: u32) -> Self {
        SimpleEntity {
            player_id,
            x,
            y,
            name,
            hp,
        }
    }
}

#[allow(dead_code)]
pub fn get_tag() -> PacketTag {
    PacketTag::SimpleEntity
}
