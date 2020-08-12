use crate::raknet::protocol::PacketId;
use crate::raknet::utils::buffer::PacketBufferWrite;

pub struct UnconnectedPong {
    packet_id: u8,
    timestamp: u128,
    server_id: u64,
}

impl UnconnectedPong {
    pub fn create(timestamp: u128, server_id: u64) -> UnconnectedPong {
        UnconnectedPong {
            packet_id: PacketId::UnconnectedPong as u8,
            timestamp,
            server_id,
        }
    }

    pub fn encode(&self, mut binary: Vec<u8>) -> Vec<u8> {
        binary.push(self.packet_id);
        binary.push_u64(self.timestamp as u64);
        binary.push_u64(self.server_id);
        binary.push_magic();
        binary.push_string(format!(
            "MCPE;{};{};{};{};{};{};{};{};{};{};{};",
            "Limonite",     // motd
            407,            // protocol
            "1.16.10",      // version
            0,              // online players
            20,             // max players
            self.server_id, // server_id
            "world",        // world name
            "Survival",     // gamemode
            1,              // is limited to switch
            19132,          // ipv4 port
            19133,          // ipv6 port
        ));

        return binary;
    }
}