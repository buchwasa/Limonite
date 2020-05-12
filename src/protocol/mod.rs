use crate::protocol::packet::PacketFlags;

pub mod client;
pub mod handler;
pub mod packet;

pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x0, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

pub const RAKNET_VERSION: u8 = 9;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PacketId {
    Unknown = 0xff,
    UnconnectedPing = 0x01,
    UnconnectedPong = 0x1c,
    ConnectionRequest1 = 0x05,
    ConnectionReply1 = 0x06,
    IncompatibleProtocolVersion = 0x1a,
    ConnectionRequest2 = 0x07,
    ConnectionReply2 = 0x08,
    ConnectionRequest = 0x09,
    ConnectionRequestAccepted = 0x10,
    ConnectedPing = 0x00,
    ConnectedPong = 0x03,
}

impl From<u8> for PacketId {
    fn from(num: u8) -> Self {
        match num {
            0x01 => PacketId::UnconnectedPing,
            0x1c => PacketId::UnconnectedPong,
            0x05 => PacketId::ConnectionRequest1,
            0x06 => PacketId::ConnectionReply1,
            0x1a => PacketId::IncompatibleProtocolVersion,
            0x07 => PacketId::ConnectionRequest2,
            0x08 => PacketId::ConnectionReply2,
            0x09 => PacketId::ConnectionRequest,
            0x10 => PacketId::ConnectionRequestAccepted,
            0x00 => PacketId::ConnectedPing,
            0x03 => PacketId::ConnectedPong,
            _ => PacketId::Unknown,
        }
    }
}
