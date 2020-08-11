use crate::raknet::protocol::client::Client;
use std::net::{UdpSocket};

use crate::raknet::protocol::handler::Handler;
use rand::random;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct Server {
    pub server_id: u64,
    /// clients connected to the server <ip:port, Client>
    pub clients: HashMap<String, Client>,
    /// bind address ip:port
    pub bind_to: String,
    /// time server has started
    pub start: SystemTime,
    /// Server socket
    pub sock: Option<UdpSocket>,
}

impl Server {
    pub fn new(bind_to: String) -> Server {
        Server {
            server_id: random::<u64>(),
            clients: HashMap::default(),
            bind_to,
            start: SystemTime::now(),
            sock: None,
        }
    }

    pub fn start(&mut self) {
        let mut buff: [u8; 2048] = [0; 2048];
        self.sock = Some(
            UdpSocket::bind(self.bind_to.clone())
                .expect(format!("Failed to bind {}!", self.bind_to).as_str()),
        );
        loop {
            let (len, src) = self.sock.as_ref().unwrap().recv_from(&mut buff).unwrap();
            self.handle_packet(&buff[0..len], src);
        }
    }
}
