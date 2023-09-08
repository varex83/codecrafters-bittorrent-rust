use anyhow::Result;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub struct Peer {
    ip: Ipv4Addr,
    port: u16,
    id: Option<Vec<u8>>,
}

impl Peer {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Peer { ip, port, id: None }
    }

    pub fn handshake(&mut self, info_hash: Vec<u8>, peer_id: Vec<u8>) -> Result<()> {
        let mut tcp = TcpStream::connect((self.ip, self.port)).unwrap();

        let hs = Self::encode_handshake(info_hash, peer_id);
        tcp.write(&hs).unwrap();

        let mut handshake = vec![0; 68];

        tcp.read_exact(&mut handshake).unwrap();

        let peer_id = Self::decode_handshake(handshake);

        self.id = Some(peer_id.clone());

        Ok(())
    }

    pub fn encode_handshake(info_hash: Vec<u8>, peer_id: Vec<u8>) -> Vec<u8> {
        let mut handshake = Vec::new();

        handshake.extend(b"19BitTorrent protocol");
        handshake.extend(vec![0; 8]);
        handshake.extend(info_hash);
        handshake.extend(peer_id);

        handshake
    }

    /// Returns the peer_id from a handshake
    pub fn decode_handshake(handshake: Vec<u8>) -> Vec<u8> {
        let hs_size = handshake.len();

        handshake[hs_size - 20..].to_vec()
    }

    pub fn get_id(&self) -> Option<Vec<u8>> {
        self.id.clone()
    }
}
