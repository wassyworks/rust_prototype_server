use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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

            let read_body_result = self.read_string().await;
            if read_body_result.is_err() || !self.is_open {
                println!("stop reading serial:{}", self.serial);
                return Ok(());
            }

            // レスポンス送信テスト
            let mut buffer = [0; 1024];
            let header_size = self.write_header_to_buffer(&mut buffer, 100);
            let packet_size =
                self.write_string_to_buffer(&mut buffer, "Response 日本語テスト", header_size);
            println!("send packet_size: {} ", packet_size);
            self.send(&buffer, packet_size).await?;
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

    pub fn write_header_to_buffer(&self, target_buffer: &mut [u8], packet_id: u16) -> usize {
        Self::copy_to_buffer(&self, target_buffer, &packet_id.to_be_bytes(), 0)
    }

    // 文字列書き込みサンプル
    pub fn write_string_to_buffer(
        &self,
        target_buffer: &mut [u8],
        string: &str,
        offset: usize,
    ) -> usize {
        Self::copy_to_buffer(&self, target_buffer, string.as_bytes(), offset)
    }

    fn copy_to_buffer(&self, target_buffer: &mut [u8], src_bytes: &[u8], offset: usize) -> usize {
        for (index, v) in src_bytes.iter().enumerate() {
            target_buffer[index + offset] = v.clone();
        }
        src_bytes.len() + offset
    }

    // 送信
    pub async fn send(&mut self, buffer: &[u8], size: usize) -> Result<(), std::io::Error> {
        self.stream.writable().await?;
        self.stream.write(&buffer[0..size]).await?;
        Ok(())
    }
}
