mod logger;
mod simple_counter;
mod socket_manager;

use std::error::Error;

#[path = "packets/simple_entity.rs"]
mod simple_entity;

#[path = "packets/simple_entity_list.rs"]
mod simple_entity_list;

#[path = "packets/packet_base.rs"]
mod packet_base;
#[path = "packets/packet_tag.rs"]
mod packet_tag;

fn serialize_deserialize_test() {
    // シリアライズ、デシリアライズテスト
    let entity = simple_entity::SimpleEntity::new(
        1000100,
        5.5555,
        54.129,
        "テストプレイヤー".to_string(),
        250000,
    );
    let encoded = bincode::serialize(&entity).unwrap();
    let decoded = bincode::deserialize::<simple_entity::SimpleEntity>(&encoded).unwrap();
    println!("decoded:{:?}", decoded);

    let mut entities: Vec<simple_entity::SimpleEntity> = Vec::new();
    for i in 0..5 {
        entities.push(simple_entity::SimpleEntity::new(
            1000100 + i,
            5.5555,
            54.129,
            format!("テストプレイヤー{}", i).to_string(),
            250000,
        ));
    }
    let entity_list = simple_entity_list::SimpleEntityList::new(entities);
    let encoded = bincode::serialize(&entity_list).unwrap();
    let decoded = bincode::deserialize::<simple_entity_list::SimpleEntityList>(&encoded).unwrap();
    println!("decoded:{:?}", decoded);
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    // ログ出力テスト
    let mut log = logger::Logger::new();
    log.logging();

    serialize_deserialize_test();

    // サーバテスト
    // socket_manager::start_accepting(44000).await;

    log.remove_log("foo.txt");
    // Ok(())
}
