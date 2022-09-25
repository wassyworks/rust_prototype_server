mod logger;
mod socket_manager;

use std::{error::Error, sync::Arc};

#[path = "packets/simple_entity.rs"]
mod simple_entity;

#[path = "packets/simple_entity_list.rs"]
mod simple_entity_list;

#[path = "packets/packet_base.rs"]
mod packet_base;
#[path = "packets/packet_tag.rs"]
mod packet_tag;

mod client;
mod client_manager;
mod dispatcher;
mod simple_entity_manager;
mod simple_entity_receiver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ログ出力テスト
    let mut log = logger::Logger::new();
    log.logging().await;
    log.remove_log("foo.txt").await?;

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
    // --

    let mut dispatcher = dispatcher::Dispatcher::new();
    let mut manager = simple_entity_manager::SimpleEntityManager::new();
    let simple_entity_receiver = simple_entity_receiver::SimpleEntityReceiver::new();
    dispatcher.add(
        packet_tag::PacketTag::SimpleEntityList,
        simple_entity_receiver::make_receive_fn(&mut manager),
    );

    let mut client_manager = client_manager::ClientManager::new();
    // サーバテスト
    socket_manager::start_accepting(44000, Arc::new(dispatcher)).await?;

    Ok(())
}
