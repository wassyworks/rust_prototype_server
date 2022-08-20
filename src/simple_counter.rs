#[derive(Debug)]
pub struct SimpleCounter {
    count: u32,
}

#[allow(dead_code)]
impl SimpleCounter {
    #[allow(dead_code)]
    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn new(cnt: u32) -> SimpleCounter {
        SimpleCounter { count: cnt }
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn add(&mut self) -> u32 {
        self.count += 1;
        self.count
    }
}
