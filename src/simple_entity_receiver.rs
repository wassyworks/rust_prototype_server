pub struct SimpleEntityReceiver;

impl SimpleEntityReceiver {
    pub fn new() -> SimpleEntityReceiver {
        SimpleEntityReceiver {}
    }

    pub fn make_receive_fn(&self) -> fn(&[u8]) {
        |data: &[u8]| println!("simple entity receiver: received")
    }
}
