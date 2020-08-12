use crate::raknet::protocol::PacketId;
use crate::raknet::utils::buffer::PacketBufferWrite;
use std::net::SocketAddr;

pub struct ConnectionRequestAccepted {
    packet_id: u8,
    address: SocketAddr,
    //system_addresses: Vec<SocketAddr> TODO
    ping: u64,
    pong: u64,
}

impl ConnectionRequestAccepted {
    pub fn create(address: SocketAddr, ping: u64, pong: u64) -> ConnectionRequestAccepted {
        ConnectionRequestAccepted {
            packet_id: PacketId::ConnectionRequestAccepted as u8,
            address,
            ping,
            pong,
        }
    }

    pub fn encode(&self, mut binary: Vec<u8>) -> Vec<u8> {
        binary.push(self.packet_id);
        binary.push_address(self.address);
        binary.push_u16(0);
        for _ in 1..20 { //MCPE uses 20, while RakNet uses 10.
            binary.push_address("0.0.0.0:0".parse().unwrap());
        }
        binary.push_u64(self.ping);
        binary.push_u64(self.pong);

        return binary;
    }
}