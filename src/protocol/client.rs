use std::net::SocketAddr;

pub struct Client {
    mtu_size: i16,
    relationship: Option<SocketAddr>,
}

impl Client {
    pub fn new(mtu_size: i16) -> Client {
        Client {
            mtu_size,
            relationship: None,
        }
    }

    pub fn mtu_size(&self) -> i16 {
        self.mtu_size
    }

    pub fn set_relationship(&mut self, relationship: SocketAddr) {
        self.relationship = Some(relationship);
    }
}
