use async_std::{
    fs::File,
    fs::{remove_file, OpenOptions},
    prelude::*,
};

pub struct Logger {
    opened_file: Option<File>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { opened_file: None }
    }
    async fn open(path: &str) -> Result<File, std::io::Error> {
        let open_result = OpenOptions::new().append(true).open(path).await;
        match open_result {
            Ok(file) => {
                return Ok(file);
            }
            Err(_err) => {
                println!("file create. {}", path);
                return File::create(path).await;
            }
        }
    }

    pub fn remove_log(&self, path: &str) {
        async_std::task::block_on(async move {
            match remove_file(path).await {
                Ok(_) => {
                    println!("file removed. {}", path);
                }
                _ => {
                    println!("failed to file remove. {}", path);
                }
            }
        });
    }

    pub fn logging(&mut self) {
        async_std::task::block_on(async move {
            if self.opened_file.is_none() {
                let result = Self::open("foo.txt").await;
                match result {
                    Ok(file) => {
                        self.opened_file = Some(file);
                    }
                    Err(err) => {
                        println!("logging open file error. {}", err.to_string());
                    }
                }
            }
            let result = self
                .opened_file
                .as_mut()
                .unwrap()
                .write_all("ロガーテスト\n".as_bytes())
                .await;
            if result.is_err() {
                println!("failed to write file.");
            }
        });
    }
}
