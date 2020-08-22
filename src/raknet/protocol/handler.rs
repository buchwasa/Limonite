use crate::raknet::utils::buffer::PacketBufferRead;
use crate::raknet::utils::buffer::PacketBufferWrite;
use crate::raknet::protocol::client::Client;
use crate::raknet::protocol::packet::{
    EncapsulatedPacket, PacketFlags, PacketInfo, PacketType, Reliability,
};
use crate::raknet::protocol::{PacketId, RAKNET_VERSION};
use crate::server::Server;
use std::net::{SocketAddr};
use crate::raknet::protocol::outbound::unconnectedpong::UnconnectedPong;
use crate::raknet::protocol::outbound::incompatibleprotocolversion::IncompatibleProtocolVersion;
use crate::raknet::protocol::outbound::connectionreply1::ConnectionReply1;
use crate::raknet::protocol::outbound::connectionreply2::ConnectionReply2;
use crate::raknet::protocol::outbound::connectionrequestaccepted::ConnectionRequestAccepted;

pub trait Handler {
    fn handle_packet(&mut self, packet: &[u8], src: SocketAddr);
}

//TODO: Split up handler.
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
                let pong_packet = UnconnectedPong::create(self.start.elapsed().unwrap().as_millis(), self.server_id);
                resp = pong_packet.encode(resp.clone());
            }
            PacketId::ConnectionRequest1 => {
                let raknet_version = packet_bytes[17];
                let mtu_size = packet_bytes[19..].len() as i16;
                if raknet_version != RAKNET_VERSION {
                    debug!(
                        "{} has an incompatible raknet version ({})",
                        src.to_string(),
                        raknet_version
                    );
                    let ipv_packet = IncompatibleProtocolVersion::create(raknet_version, self.server_id);
                    resp = ipv_packet.encode(resp.clone());
                } else {
                    let reply_1 = ConnectionReply1::create(self.server_id, 0x00, mtu_size);
                    resp = reply_1.encode(resp.clone());
                }
                self.clients
                    .insert(src.to_string(), Client::new(src.clone(), mtu_size));
            }
            PacketId::ConnectionRequest2 => {
                let client = self.clients.get_mut(&src.to_string()).unwrap();
                client.set_relationship(packet_bytes.read_address(17));
                let reply_2 = ConnectionReply2::create(self.server_id, client.mtu_size(), 0x00);
                resp = reply_2.encode(resp.clone());
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
                let connection_request_accepted = ConnectionRequestAccepted::create(src, time_since_start, self.start.elapsed().unwrap().as_millis() as u64);
                resp = connection_request_accepted.encode(resp.clone());
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
                resp
            }
            _ => {
                warn!(
                    "Could not handle Packet: 0x{:02x} ({:?})",
                    packet_info.packet_id().unwrap() as u8,
                    packet_info.packet_id(),
                );
                println!("{:#?}\nlen: {}", packet_bytes, packet_bytes.len());
            }
        }
        if resp.len() > 0 {
            self.sock.as_ref().unwrap().send_to(&resp, src);
        }
    }
}
