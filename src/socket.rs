use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[path = "packets/sample_entity_packet.rs"]
mod sample_entity_packet;

const BUFFER_SIZE: usize = 1024;

pub struct Socket {
    stream: TcpStream,
    serial: u64,
    is_open: bool,
    send_buffer: [u8; BUFFER_SIZE],
}

impl Socket {
    pub fn new(stream: TcpStream, serial: u64) -> Socket {
        Socket {
            stream,
            serial,
            is_open: true,
            send_buffer: [0; BUFFER_SIZE],
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

            let read_body_result = self.read_string().await;
            if read_body_result.is_err() || !self.is_open {
                println!("stop reading serial:{}", self.serial);
                return Ok(());
            }

            // レスポンス送信テスト
            // self.send_string("Response 日本語テスト").await?;
            self.send_struct().await?;
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

    // 文字列読み込みサンプル
    async fn read_string(&mut self) -> Result<(), std::io::Error> {
        let mut buf = [0; BUFFER_SIZE];

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

    // 送信
    #[allow(dead_code)]
    pub async fn send_simple_string(&mut self, string: &str) -> Result<(), std::io::Error> {
        let header_size = self.write_header_to_buffer(100);
        match self.write_simple_string_to_buffer(string, header_size) {
            Ok(packet_size) => {
                println!("send packet_size: {} ", packet_size);
                self.write_buffer(packet_size).await
            }
            Err(err) => {
                println!("{}", err);
                Ok(())
            }
        }
    }

    // 雑に構造体を作って送る
    pub async fn send_struct(&mut self) -> Result<(), std::io::Error> {
        let header_size = self.write_header_to_buffer(100);
        let entity = sample_entity_packet::SampleEntity::new(
            1000100,
            5.5555,
            54.129,
            "テストプレイヤー".to_string(),
            250000,
        );
        let encoded = bincode::serialize(&entity).unwrap();
        let total_size = self.copy_to_buffer(&encoded, header_size);

        self.write_buffer(total_size).await
    }

    // バッファは先頭から書き換え
    fn write_header_to_buffer(&mut self, packet_id: u16) -> usize {
        Self::copy_to_buffer(self, &packet_id.to_be_bytes(), 0)
    }

    // 文字列書き込みサンプル
    fn write_simple_string_to_buffer(
        &mut self,
        string: &str,
        offset: usize,
    ) -> Result<usize, String> {
        let bytes = string.as_bytes();
        if bytes.len() > BUFFER_SIZE - offset {
            return Err(format!(
                "not enough buffer size. str: {} strbytelength:{}, offset{}",
                string,
                bytes.len(),
                offset
            ));
        }
        Ok(Self::copy_to_buffer(self, bytes, offset))
    }

    fn copy_to_buffer(&mut self, src_bytes: &[u8], offset: usize) -> usize {
        for (index, v) in src_bytes.iter().enumerate() {
            self.send_buffer[index + offset] = v.clone();
        }
        src_bytes.len() + offset
    }

    // 実際のデータ送信
    async fn write_buffer(&mut self, size: usize) -> Result<(), std::io::Error> {
        self.stream.writable().await?;
        self.stream.write(&self.send_buffer[0..size]).await?;
        Ok(())
    }
}
