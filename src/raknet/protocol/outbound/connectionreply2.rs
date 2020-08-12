use crate::raknet::protocol::PacketId;
use crate::raknet::utils::buffer::PacketBufferWrite;

pub struct ConnectionReply2 {
    packet_id: u8,
    server_id: u64,
    mtu_size: i16,
    server_security: u8,
}

impl ConnectionReply2 {
    pub fn create(server_id: u64, mtu_size: i16, server_security: u8) -> ConnectionReply2 {
        ConnectionReply2 {
            packet_id: PacketId::ConnectionReply2 as u8,
            server_id,
            mtu_size,
            server_security,
        }
    }

    pub fn encode(&self, mut binary: Vec<u8>) -> Vec<u8> {
        binary.push(self.packet_id);
        binary.push_magic();
        binary.push_u64(self.server_id);
        binary.push_i16(self.mtu_size);
        binary.push(self.server_security);

        return binary;
    }
}