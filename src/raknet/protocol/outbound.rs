use crate::raknet::protocol::{PacketId, ADDRESS_COUNT};
use crate::raknet::utils::buffer::PacketBufferWrite;
use std::net::SocketAddr;

pub struct UnconnectedPong {
    packet_id: u8,
    timestamp: u128,
    server_id: u64,
}

pub struct ConnectionReply1 {
    packet_id: u8,
    server_id: u64,
    server_security: u8,
    mtu_size: i16,
}

pub struct ConnectionReply2 {
    packet_id: u8,
    server_id: u64,
    mtu_size: i16,
    server_security: u8,
}

pub struct ConnectionRequestAccepted {
    packet_id: u8,
    address: SocketAddr,
    //system_addresses: Vec<SocketAddr> TODO
    ping: u64,
    pong: u64,
}

pub struct IncompatibleProtocolVersion {
    packet_id: u8,
    raknet_version: u8,
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
            408,            // protocol
            "1.16.20",      // version
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
        for _ in 1..ADDRESS_COUNT {
            binary.push_address("0.0.0.0:0".parse().unwrap());
        }
        binary.push_u64(self.ping);
        binary.push_u64(self.pong);

        return binary;
    }
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
