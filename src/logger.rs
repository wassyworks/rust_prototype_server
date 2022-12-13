use async_std::{fs::remove_file, fs::File, fs::OpenOptions, prelude::*, sync::Mutex};
use chrono::prelude::*;
use std::sync::Arc;

pub struct Logger {
    opened_file: SharedFile,
}
type SharedFile = Arc<Mutex<File>>;

impl Logger {
    pub async fn new(path: &str) -> Logger {
        let open_result = OpenOptions::new().append(true).open(path).await;
        match open_result {
            Ok(file) => {
                println!("file open {}", path);
                Logger {
                    opened_file: Arc::new(Mutex::new(file)),
                }
            }
            Err(_err) => {
                println!("file create. {}", path);
                match File::create(path).await {
                    Ok(file) => Logger {
                        opened_file: Arc::new(Mutex::new(file)),
                    },
                    _ => {
                        println!("error");
                        panic!();
                    }
                }
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

    pub async fn log(&mut self, string: String) {
        let file = self.opened_file.clone();
        async_std::task::spawn(async move {
            let mut f = file.lock().await;
            let now = Utc::now().format("%Y%m%dT%H%M%S");
            let result = f
                .write_all(format!("{},{}\n", now.to_string(), string.as_str()).as_bytes())
                .await;
            println!("{:?},{}", now.to_string(), string);
            if result.is_err() {
                println!("failed to write file.");
            }
        });
    }
}
