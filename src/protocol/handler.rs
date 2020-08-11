use crate::utils::buffer::PacketBufferRead;
use crate::utils::buffer::PacketBufferWrite;
use crate::protocol::client::Client;
use crate::protocol::packet::{
    EncapsulatedPacket, PacketFlags, PacketInfo, PacketType, Reliability,
};
use crate::protocol::{PacketId, RAKNET_VERSION};
use crate::server::Server;
use log::{debug, error, info, warn};
use std::net::{SocketAddr, SocketAddrV4};
use std::process::exit;

pub trait Handler {
    fn handle_packet(&mut self, packet: &[u8], src: SocketAddr);
}

impl Handler for Server {
    fn handle_packet(&mut self, packet_bytes: &[u8], src: SocketAddr) {
        let packet_info = PacketInfo::from_bytes(&packet_bytes);
        let mut resp: Vec<u8> = Vec::new();
        if packet_info.packet_id().is_none() {
            debug!("Received id-less packet, ignoring?");
            return;
        }
        debug!(
            "Received 0x{:02x} ({:?}) packet to handle from {} (encapsulated: {})",
            packet_info.packet_id().unwrap() as u8,
            packet_info.packet_id().unwrap(),
            src.to_string(),
            packet_info.is_encapsulated()
        );
        match packet_info.packet_id().unwrap() {
            PacketId::UnconnectedPing => {
                resp.push(PacketId::UnconnectedPong as u8);
                resp.push_u64(self.start.elapsed().unwrap().as_millis() as u64);
                resp.push_u64(self.server_id);
                resp.push_magic();
                resp.push_string(format!(
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
            }
            PacketId::ConnectionRequest1 => {
                let raknet_version = packet_bytes[17];
                let mtu_size = packet_bytes[19..].len() as i16;
                if raknet_version != RAKNET_VERSION {
                    println!( //TODO: Use log
                        "{} has an incompatible raknet protocol ({})",
                        src.to_string(),
                        raknet_version
                    );
                    resp.push(PacketId::IncompatibleProtocolVersion as u8);
                    resp.push(raknet_version);
                    resp.push_magic();
                    resp.push_u64(self.server_id);
                } else {
                    resp.push(PacketId::ConnectionReply1 as u8);
                    resp.push_magic();
                    resp.push_u64(self.server_id);
                    resp.push(0x00);
                    resp.push_i16(mtu_size);
                }
                self.clients
                    .insert(src.to_string(), Client::new(src.clone(), mtu_size));
            }
            PacketId::ConnectionRequest2 => {
                let client = self.clients.get_mut(&src.to_string()).unwrap();
                client.set_relationship(packet_bytes.read_address(17));
                resp.push(PacketId::ConnectionReply2 as u8);
                resp.push_magic();
                resp.push_u64(self.server_id);
                resp.push_address(src);
                resp.push_i16(client.mtu_size());
                resp.push(0x00);
            }
            PacketId::ConnectionRequest => {
                let encapsulated_packet = EncapsulatedPacket::decode(&packet_bytes);
                if encapsulated_packet.is_err() {
                    error!("Failed to decode encapsulated Packet");
                    return;
                }
                let encapsulated_packet = encapsulated_packet.unwrap();
                let guid = encapsulated_packet.body.clone().unwrap().read_u64(1);
                let time_since_start = encapsulated_packet.body.unwrap().read_u64(9);
                self.clients
                    .get_mut(src.to_string().as_str())
                    .unwrap()
                    .set_guid(guid);
                resp.push(PacketId::ConnectionRequestAccepted as u8);
                resp.push_address(src);
                resp.push_u16(0);
                resp.push_address(
                    self.clients
                        .get(&src.to_string())
                        .unwrap()
                        .relationship()
                        .unwrap(),
                );
                for _ in 1..10 {
                    // no idea what this could be
                    resp.push_address("0.0.0.0:0".parse().unwrap());
                }
                resp.push_u64(time_since_start);
                resp.push_u64(self.start.elapsed().unwrap().as_millis() as u64);
                let pk = EncapsulatedPacket {
                    packet_type: PacketType {
                        is_connected_to_peer: true,
                        is_ack: false,
                        is_nak: false,
                        is_pair: false,
                        is_continuous_send: false,
                        b_and_as: true,
                    },
                    sequence_number: Some(0),
                    record_count: None,
                    packet_flags: Some(PacketFlags {
                        reliability: Reliability::Unreliable,
                        has_split_packet: false,
                    }),
                    reliable_packets: None,
                    sequence_number_range: None,
                    body: Some(resp.clone()),
                };
                resp = pk.encode();
            }
            _ => {
                warn!(
                    "Could not handle Packet: 0x{:02x} ({:?})",
                    packet_info.packet_id().unwrap() as u8,
                    packet_info.packet_id(),
                );
                println!("{:#?}\nlen: {}", packet_bytes, packet_bytes.len());
                exit(0);
            }
        }
        if resp.len() > 0 {
            self.sock.as_ref().unwrap().send_to(&resp, src);
        }
    }
}
