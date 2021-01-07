use crate::protocol::client::Client;
use std::net::{UdpSocket};

use crate::protocol::handler::Handler;
use rand::random;
use std::collections::HashMap;
use std::time::SystemTime;
use crate::protocol::RakNetSettings;

pub struct Server {
    /// unique server id
    pub server_id: u64,
    /// clients connected to the server <ip:port, Client>
    pub clients: HashMap<String, Client>,
    /// raknet settings
    pub raknet_settings: RakNetSettings,
    /// time server has started
    pub start: SystemTime,
    /// Server socket
    pub sock: Option<UdpSocket>,
}

impl Server {
    pub fn new(raknet_settings: RakNetSettings) -> Server {
        Server {
            server_id: random::<u64>(),
            clients: HashMap::default(),
            raknet_settings,
            start: SystemTime::now(),
            sock: None,
        }
    }

    pub fn start(&mut self) {
        let mut buff: [u8; 2048] = [0; 2048];
        self.sock = Some(
            UdpSocket::bind(self.raknet_settings.get_address())
                .expect(format!("Failed to bind to port {}", self.raknet_settings.get_port()).as_str()),
        );
        loop {
            let (len, src) = self.sock.as_ref().unwrap().recv_from(&mut buff).unwrap();
            self.handle_packet(&buff[0..len], src);
        }
    }
}
