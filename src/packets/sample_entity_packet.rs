use serde::{Deserialize, Serialize};

// シリアライズテスト用構造体
#[derive(Serialize, Deserialize, Debug)]
pub struct SampleEntity {
    player_id: u64,
    x: f32,
    y: f32,
    name: String,
    hp: u32,
}

impl SampleEntity {
    pub fn new(player_id: u64, x: f32, y: f32, name: String, hp: u32) -> Self {
        SampleEntity {
            player_id,
            x,
            y,
            name,
            hp,
        }
    }
}
