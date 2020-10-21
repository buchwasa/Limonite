use std::net::SocketAddr;
use std::time::SystemTime;

pub struct Client {
    addr: SocketAddr,
    mtu_size: i16,
    last_update: SystemTime,
    relationship: Option<SocketAddr>,
    guid: Option<u64>,
}

impl Client {
    pub fn new(addr: SocketAddr, mtu_size: i16) -> Client {
        Client {
            addr,
            mtu_size,
            guid: None,
            relationship: None,
            last_update: SystemTime::now(),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn mtu_size(&self) -> i16 {
        self.mtu_size
    }

    pub fn last_update(&self) -> SystemTime {
        self.last_update
    }

    pub fn guid(&self) -> Option<u64> {
        self.guid
    }

    pub fn set_guid(&mut self, guid: u64) {
        self.guid = Some(guid);
    }

    pub fn has_guid(&self) -> bool {
        match self.guid {
            Some(_) => true,
            None => false,
        }
    }

    pub fn relationship(&self) -> Option<SocketAddr> {
        self.relationship
    }

    pub fn set_relationship(&mut self, relationship: SocketAddr) {
        self.relationship = Some(relationship);
    }

    pub fn has_relationship(&self) -> bool {
        match self.relationship {
            Some(_) => true,
            None => false,
        }
    }
}
