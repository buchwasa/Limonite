use crate::raknet::utils::buffer::{PacketBufferRead, PacketBufferWrite};
use crate::raknet::protocol::PacketId;
use std::convert::{TryFrom, TryInto};
use std::io::ErrorKind;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Reliability {
    // TODO: Complete
    Reliable = 0b010,        // 2
    ReliableOrdered = 0b011, // 3 - This one is not fully implemented
    Unreliable = 0b000,      // 0
}

impl TryFrom<u8> for Reliability {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Reliability::Unreliable),
            0b010 => Ok(Reliability::Reliable),
            0b011 => Ok(Reliability::ReliableOrdered),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct PacketType {
    pub is_connected_to_peer: bool,
    pub is_ack: bool,
    pub is_nak: bool,
    pub is_pair: bool,
    pub is_continuous_send: bool,
    pub b_and_as: bool,
}

#[derive(Debug)]
pub struct PacketFlags {
    pub reliability: Reliability,
    pub has_split_packet: bool,
}

#[derive(Debug)]
pub struct PacketInfo {
    packet_id: Option<PacketId>,
    encapsulated: bool,
}

#[derive(Debug)]
pub struct EncapsulatedPacket {
    pub packet_type: PacketType,
    /// Packet sequence number, is decoded as an u24
    pub sequence_number: Option<u32>,
    /// Set when either NAK or ACK Packet
    pub record_count: Option<u16>,
    pub packet_flags: Option<PacketFlags>,
    /// decoded as an u24
    /// Only set when packet is Reliable
    pub reliable_packets: Option<u32>,
    /// Only set when ACK packet
    pub sequence_number_range: Option<SequenceNumberRange>,
    /// Packet Body, max size of u16::MAX (in bits)
    /// Not set if ACK
    pub body: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct SequenceNumberRange {
    pub max_equals_to_min: bool,
    /// decoded as an u24
    pub sequence_number_min: u32,
    /// decoded as an u24
    /// Depends on `max_equals_to_min`t
    pub sequence_number_max: Option<u32>,
}

impl PacketFlags {
    pub fn from_u8(byte: u8) -> Result<Self, std::io::Error> {
        Ok(PacketFlags {
            reliability: Reliability::try_from(byte >> 5).map_err(|_| {
                std::io::Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "packet reliability {} ({:03b}) is not implemented",
                        byte >> 5,
                        byte >> 5
                    ),
                )
            })?,
            has_split_packet: (byte & (1 << 4)) != 0,
        })
    }

    pub fn to_u8(&self) -> u8 {
        ((self.reliability as u8) << 5) | if self.has_split_packet { 1 << 4 } else { 0 }
    }
}

impl PacketType {
    pub fn from_u8(byte: u8) -> Self {
        PacketType {
            is_connected_to_peer: (byte & 1 << 7) != 0,
            is_ack: (byte & (1 << 6)) != 0,
            is_nak: (byte & (1 << 5)) != 0,
            is_pair: (byte & (1 << 4)) != 0,
            is_continuous_send: (byte & (1 << 3)) != 0,
            b_and_as: (byte & (1 << 2)) != 0,
        }
    }
    pub fn to_u8(&self) -> u8 {
        #[rustfmt::skip]
        let num =
            if self.is_connected_to_peer { 1 << 7 } else { 0 } |
            if self.is_ack               { 1 << 6 } else { 0 } |
            if self.is_nak               { 1 << 5 } else { 0 } |
            if self.is_pair              { 1 << 4 } else { 0 } |
            if self.is_continuous_send   { 1 << 3 } else { 0 } |
            if self.b_and_as             { 1 << 2 } else { 0 };
        num
    }
}

impl EncapsulatedPacket {
    pub fn is_ack(&self) -> bool {
        self.packet_type.is_ack
    }

    pub fn is_nak(&self) -> bool {
        self.packet_type.is_nak
    }

    pub fn has_b_and_as(&self) -> bool {
        self.packet_type.is_ack
    }

    pub fn needs_b_and_as(&self) -> bool {
        self.packet_type.b_and_as
    }

    pub fn is_pair(&self) -> bool {
        self.packet_type.is_pair
    }

    pub fn is_connected_to_peer(&self) -> bool {
        self.packet_type.is_connected_to_peer
    }

    pub fn is_continuous_send(&self) -> bool {
        self.packet_type.is_continuous_send
    }

    pub fn decode(bytes: &[u8]) -> Result<EncapsulatedPacket, std::io::Error> {
        let packet_type = PacketType::from_u8(bytes[0]);
        let mut sequence_number = None;
        let mut record_count = None;
        let mut packet_flags = None;
        let mut reliable_packets = None;
        let mut sequence_number_range = None;
        let mut body = None;
        if packet_type.is_ack {
            reliable_packets = None;
            packet_flags = None;
            record_count = Some(u16::from_be_bytes(bytes[1..3].try_into().unwrap()));
            sequence_number = None;
            body = None;
            let mut sequence_number_max = [0u8; 4];
            if bytes[3] == 1 {
                sequence_number_max[0..3].clone_from_slice(&bytes[7..10]);
            }
            let sequence_number_range = Some(SequenceNumberRange {
                max_equals_to_min: bytes[3] == 1,
                sequence_number_min: bytes.read_u24(4),
                sequence_number_max: if bytes[3] == 1 {
                    Some(bytes.read_u24(7))
                } else {
                    None
                },
            });
            body = None;
        } else {
            let packet_flags_raw = PacketFlags::from_u8(bytes[4]).unwrap();
            packet_flags = Some(PacketFlags::from_u8(bytes[4]).unwrap());
            if packet_flags_raw.reliability != Reliability::Unreliable {
                let reliable_packets = Some(bytes.read_u24(7));
            }
            #[rustfmt::skip]
            let header_size =
                if packet_flags_raw.reliability != Reliability::Unreliable {
                    10
                } else {
                    7
                };
            body = Some(bytes[header_size..].to_vec());
            record_count = None;
            sequence_number = Some(bytes.read_u24(1));
            sequence_number_range = None;
        }

        Ok(EncapsulatedPacket {
            packet_type,
            sequence_number,
            record_count,
            packet_flags,
            reliable_packets,
            sequence_number_range,
            body,
        })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        packet.push(self.packet_type.to_u8());
        if self.packet_type.is_ack {
            packet.push_u16(self.record_count.expect("Missing data for ACK"));
            let snr = self
                .sequence_number_range
                .as_ref()
                .expect("Missing data for ACK");
            packet.push(if snr.max_equals_to_min { 1 } else { 0 });
            packet.push_u24(snr.sequence_number_min);
            if !snr.max_equals_to_min {
                packet.push_u24(snr.sequence_number_max.expect("Missing data for ACK"));
            }
            packet
        } else {
            packet.push_u24(self.sequence_number.expect("Missing data"));
            packet.push(self.packet_flags.as_ref().unwrap().to_u8());
            packet.push_u16(self.body.as_ref().unwrap().len() as u16 * 8); // octets -> bits
            if self.packet_flags.as_ref().unwrap().reliability != Reliability::Unreliable {
                packet.push_u24(self.reliable_packets.expect("Missing data"));
            }
            packet.push_slice(&self.body.as_ref().unwrap());
            packet
        }
    }
}

impl PacketInfo {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let packet_header = PacketType::from_u8(bytes[0]);
        let mut encapsulated = false;
        let mut packet_id = Some(PacketId::Unknown);
        // good enough i guess
        if PacketId::from(bytes[0]) == PacketId::Unknown {
            encapsulated = true;
            if packet_header.is_ack || packet_header.is_nak {
                packet_id = None;
            } else {
                let packet_flags = PacketFlags::from_u8(bytes[4]).unwrap();
                packet_id = Some(if packet_flags.reliability != Reliability::Unreliable {
                    PacketId::from(bytes[10])
                } else {
                    PacketId::from(bytes[7])
                });
            }
        } else {
            encapsulated = false;
            packet_id = Some(PacketId::from(bytes[0]));
        }
        PacketInfo {
            packet_id,
            encapsulated,
        }
    }

    pub fn packet_id(&self) -> Option<PacketId> {
        self.packet_id
    }

    pub fn is_encapsulated(&self) -> bool {
        self.encapsulated
    }
}
