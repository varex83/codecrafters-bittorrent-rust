use crate::parser::Parser;
use crate::parser::ValueToString;
use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::Ipv4Addr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerRequest {
    pub info_hash: String,
    pub peer_id: String,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrackerResponse {
    interval: u64,
    #[serde(with = "serde_bytes")]
    peers: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct TrackerRequestBuilder {
    request: TrackerRequest,
}

impl TrackerRequest {
    pub fn execute(&self, tracker_url: String) -> TrackerResponse {
        let client = reqwest::blocking::Client::new();

        let url = format!("{}?info_hash={}", tracker_url, &self.info_hash);

        let req = client
            .get(url)
            .query(&[("peer_id", self.peer_id.clone())])
            .query(&[("port", self.port.to_string())])
            .query(&[("uploaded", self.uploaded.to_string())])
            .query(&[("downloaded", self.downloaded.to_string())])
            .query(&[("left", self.left.to_string())])
            .query(&[("compact", self.compact.to_string())])
            .build()
            .unwrap();

        let resp = client.execute(req).unwrap().bytes().unwrap();

        TrackerResponse::from_bytes(&resp).unwrap()
    }
}

impl Default for TrackerRequest {
    fn default() -> Self {
        Self {
            info_hash: String::new(),
            peer_id: String::from("00112239945566778899"),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            compact: 1,
        }
    }
}

impl TrackerRequestBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn info_hash(mut self, info_hash: String) -> Self {
        self.request.info_hash = info_hash;
        self
    }

    pub fn peer_id(mut self, peer_id: &str) -> Self {
        self.request.peer_id = peer_id.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.request.port = port;
        self
    }

    pub fn uploaded(mut self, uploaded: u64) -> Self {
        self.request.uploaded = uploaded;
        self
    }

    pub fn downloaded(mut self, downloaded: u64) -> Self {
        self.request.downloaded = downloaded;
        self
    }

    pub fn left(mut self, left: u64) -> Self {
        self.request.left = left;
        self
    }

    pub fn compact(mut self, compact: u8) -> Self {
        self.request.compact = compact;
        self
    }

    pub fn build(self) -> TrackerRequest {
        self.request
    }
}

impl TrackerResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_bencode::from_bytes(bytes).map_err(|e| anyhow!("Failed to parse input: {}", e))
    }

    pub fn get_peers(&self) -> Result<Vec<(Ipv4Addr, u16)>> {
        let mut result = Vec::<(Ipv4Addr, u16)>::new();

        for peer in self.peers.chunks(6) {
            let mut ip = [0u8; 4];
            ip.copy_from_slice(&peer[..4]);

            let port = u16::from_be_bytes([peer[4], peer[5]]);

            result.push((Ipv4Addr::from(ip), port));
        }

        Ok(result)
    }
}
