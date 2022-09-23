pub struct Client {
    socket_id: u64,
}

impl Client {
    pub fn new(socket_id: u64) -> Client {
        Client { socket_id }
    }
}
