use crate::protocol::client::Client;
use std::net::{SocketAddr, UdpSocket};

use crate::protocol::handler::Handler;
use log::info;
use rand::random;
use std::collections::HashMap;
use std::ops::Add;
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
    /// list of blocked users <address, blocked until>
    pub blacklist: HashMap<String, SystemTime>,
}

impl Server {
    pub fn new(bind_to: String) -> Server {
        Server {
            server_id: random::<u64>(),
            clients: HashMap::default(),
            bind_to: bind_to,
            start: SystemTime::now(),
            sock: None,
            blacklist: HashMap::default(),
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
            if self.is_blocked(src) {
                info!(
                    "Client {} sent a packet, but it was ignored because they're blocked!",
                    src.ip().to_string()
                );
                continue;
            }
            self.handle_packet(&buff[0..len], src);
        }
    }

    fn block_addr(&mut self, to_block: String, time_in_seconds: u64) {
        self.blacklist.insert(
            to_block,
            SystemTime::now().add(std::time::Duration::from_secs(time_in_seconds)),
        );
    }

    fn is_blocked(&mut self, addr: SocketAddr) -> bool {
        match self.blacklist.get(addr.ip().to_string().as_str()) {
            Some(time) => {
                if time.elapsed().is_err() {
                    true
                } else {
                    self.blacklist.remove(addr.ip().to_string().as_str());
                    false
                }
            }
            None => false,
        }
    }
}
