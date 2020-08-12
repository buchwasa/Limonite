use crate::raknet::protocol::PacketId;
use crate::raknet::utils::buffer::PacketBufferWrite;

pub struct ConnectionReply1 {
    packet_id: u8,
    server_id: u64,
    server_security: u8,
    mtu_size: i16,
}

impl ConnectionReply1 {
    pub fn create(server_id: u64, server_security: u8, mtu_size: i16) -> ConnectionReply1 {
        ConnectionReply1 {
            packet_id: PacketId::ConnectionReply1 as u8,
            server_id,
            server_security,
            mtu_size,
        }
    }

    pub fn encode(&self, mut binary: Vec<u8>) -> Vec<u8> {
        binary.push(self.packet_id);
        binary.push_magic();
        binary.push_u64(self.server_id);
        binary.push(self.server_security);
        binary.push_i16(self.mtu_size);

        return binary;
    }
}