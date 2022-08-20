use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Socket {
    stream: TcpStream,
    serial: u64,
    is_open: bool,
}

impl Socket {
    pub fn new(stream: TcpStream, serial: u64) -> Socket {
        Socket {
            stream,
            serial,
            is_open: true,
        }
    }
    pub async fn start_reading(&mut self) -> Result<(), std::io::Error> {
        // Nagleアルゴリズムを切る
        self.stream.set_nodelay(true)?;

        println!("start reading serial:{}", self.serial);
        loop {
            let read_header_result = self.read_header().await;
            if read_header_result.is_err() || !self.is_open {
                println!("stop reading serial:{}", self.serial);
                return Ok(());
            }

            let read_body_result = self.read_body().await;
            if read_body_result.is_err() || !self.is_open {
                println!("stop reading serial:{}", self.serial);
                return Ok(());
            }
        }
    }

    async fn read_header(&mut self) -> Result<(), std::io::Error> {
        let header = self.stream.read_u16().await;
        match header {
            Ok(0) => {
                println!("socket closed.");
                self.is_open = false;
            }
            Ok(packet_id) => {
                println!("read header {}.", packet_id);
            }
            Err(err) => {
                println!("error occured. {}", err.to_string());
                self.is_open = false;
                return Err(err);
            }
        }
        Ok(())
    }
    async fn read_body(&mut self) -> Result<(), std::io::Error> {
        let mut buf = [0; 4096];

        let n = self.stream.read(&mut buf).await;
        match n {
            Ok(0) => {
                println!("socket closed.");
                return Ok(());
            }
            Ok(bytes) => {
                println!("read {} bytes.", bytes);
                let s = std::str::from_utf8(&buf[0..bytes]);
                match s {
                    Ok(str) => {
                        println!("received string: {}", str);
                    }
                    Err(err) => {
                        println!("received string error: {}", err.to_string());
                    }
                }
            }
            Err(err) => {
                println!("read body error occured. {}", err.to_string());
            }
        }
        Ok(())
    }
}
