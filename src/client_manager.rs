use crate::client::Client;
pub struct ClientManager {
    clients: Vec<Client>,
}

impl ClientManager {
    pub fn new() -> ClientManager {
        ClientManager {
            clients: Vec::new(),
        }
    }

    fn CreateClient(&mut self, socket_id: u64) {
        self.clients.push(Client::new(socket_id));
    }
}
