#[path = "socket.rs"]
mod socket;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub struct SocketManager {
    sockets: Vec<socket::Socket>,
    latest_serial: u64,
}

pub async fn start_accepting(port: u16) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).await?;

    println!("start accepting.");
    let socket_manager = Arc::new(Mutex::new(SocketManager::new()));
    loop {
        let (stream, _) = listener.accept().await?;
        let man = socket_manager.clone();
        println!("accepted.");
        tokio::spawn(async move {
            let result = man.lock().await.on_accepted(stream).await;
            match result {
                Err(err) => {
                    println!("acception failed. {}", err.to_string());
                }
                _ => {}
            }
        });
    }
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
