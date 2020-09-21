use crate::raknet::utils::buffer::PacketBufferRead;
use crate::raknet::utils::buffer::PacketBufferWrite;
use crate::raknet::protocol::client::Client;
use crate::raknet::protocol::packet::{
    PacketFlags, PacketInfo, PacketType, Reliability,
};
use crate::raknet::protocol::{PacketId, RAKNET_VERSION};
use crate::server::Server;
use std::net::{SocketAddr};
use crate::raknet::protocol::outbound::{
    UnconnectedPong, IncompatibleProtocolVersion, ConnectionReply1, ConnectionReply2
};

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
