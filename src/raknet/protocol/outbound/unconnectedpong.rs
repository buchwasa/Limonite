use crate::raknet::utils::buffer::PacketBufferWrite;
use crate::raknet::protocol::PacketId;

pub struct UnconnectedPong {
    pub packet_id: u8,
    pub timestamp: u128,
    pub server_id: u64,
}

impl UnconnectedPong {
    pub fn create(time: u128, id: u64) -> UnconnectedPong{
        UnconnectedPong{
            packet_id: PacketId::UnconnectedPong as u8,
            timestamp: time,
            server_id: id,
        }
    }

}