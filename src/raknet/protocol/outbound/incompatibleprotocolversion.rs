use crate::raknet::protocol::PacketId;
use crate::raknet::utils::buffer::PacketBufferWrite;

pub struct IncompatibleProtocolVersion {
    packet_id: u8,
    raknet_version: u8,
    server_id: u64,
}

impl IncompatibleProtocolVersion {
    pub fn create(raknet_version: u8, server_id: u64) -> IncompatibleProtocolVersion {
        IncompatibleProtocolVersion {
            packet_id: PacketId::IncompatibleProtocolVersion as u8,
            raknet_version,
            server_id,
        }
    }

    pub fn encode(&self, mut binary: Vec<u8>) -> Vec<u8> {
        binary.push(self.packet_id);
        binary.push(self.raknet_version);
        binary.push_magic();
        binary.push_u64(self.server_id);

        return binary;
    }
}