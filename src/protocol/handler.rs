use crate::utils::buffer::PacketBufferRead;
use crate::protocol::client::Client;
use crate::protocol::packet::PacketInfo;
use crate::protocol::{PacketId, RAKNET_VERSION};
use crate::server::Server;
use std::net::{SocketAddr};
use crate::protocol::outbound::{
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
                resp = UnconnectedPong::create(self.start.elapsed().unwrap().as_millis(), self.server_id).encode(resp.clone());
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
                    resp = IncompatibleProtocolVersion::create(raknet_version, self.server_id).encode(resp.clone());
                } else {
                    resp = ConnectionReply1::create(self.server_id, 0x00, mtu_size).encode(resp.clone());
                }
                self.clients
                    .insert(src.to_string(), Client::new(mtu_size));
            }
            PacketId::ConnectionRequest2 => {
                let client = self.clients.get_mut(&src.to_string()).unwrap();
                client.set_relationship(packet_bytes.read_address(17));
                resp = ConnectionReply2::create(self.server_id, client.mtu_size(), 0x00).encode(resp.clone());
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
            self.sock.as_ref().unwrap().send_to(&resp, src).expect("Failed to unwrap");
        }
    }
}
