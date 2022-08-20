mod logger;
mod simple_counter;
mod socket_manager;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ログ出力テスト
    let mut log = logger::Logger::new();
    log.logging().await;
    log.remove_log("foo.txt").await?;

    // サーバテスト
    socket_manager::start_accepting(44000).await?;

    Ok(())
}
