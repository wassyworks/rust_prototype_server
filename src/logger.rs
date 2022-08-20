use tokio::{
    fs::File,
    fs::{remove_file, OpenOptions},
    io::AsyncWriteExt,
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

    pub async fn remove_log(&self, path: &str) -> Result<(), std::io::Error> {
        remove_file(path).await?;
        println!("file removed. {}", path);
        Ok(())
    }

    pub async fn logging(&mut self) {
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
    }
}
