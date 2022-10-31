#[path = "socket.rs"]
mod socket;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::sync::Mutex;
use std::sync::Arc;

pub struct SocketManager {
    sockets: Vec<socket::Socket>,
    latest_serial: u64,
}

pub fn start_accepting(port: u16) {
    async_std::task::block_on(async {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).await;
        // if listener.is_err() {
        //     println!("failed to bind. ");
        //     return;
        // }

        println!("start accepting.");

        let mut new_conn = listener.as_ref().unwrap().incoming();
        let socket_manager = Arc::new(Mutex::new(SocketManager::new()));
        while let Some(result) = new_conn.next().await {
            let man = socket_manager.clone();
            println!("accepted.");
        }
    });
}

impl SocketManager {
    pub fn new() -> SocketManager {
        SocketManager {
            sockets: Vec::new(),
            latest_serial: 0,
        }
    }

    pub async fn on_accepted(&mut self, socket: TcpStream) -> Result<(), std::io::Error> {
        self.latest_serial += 1;
        self.sockets
            .push(socket::Socket::new(socket, self.latest_serial));
        println!("client connected sockets:{}", self.sockets.len());
        // パケット読み出し
        let socket = &mut self.sockets.last_mut().unwrap();
        socket.start_reading().await?;
        Ok(())
    }
}
