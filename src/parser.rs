use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_bencode::value::Value;
use sha1::{Digest, Sha1};

#[derive(Debug, Default)]
pub struct Parser {}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TorrentFile {
    pub announce: String,
    pub info: TorrentInfo,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TorrentInfo {
    pub length: i64,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
}

impl TorrentInfo {
    pub fn hash(&self) -> Result<Vec<u8>> {
        let mut hasher = Sha1::default();

        hasher.update(serde_bencode::to_bytes(self)?);

        Ok(hasher.finalize().to_vec())
    }
}

impl Parser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode_input(&mut self, input: String) -> Result<Value> {
        let input = input.as_bytes();

        let result = serde_bencode::from_bytes::<Value>(input);

        result.map_err(|e| anyhow!("Failed to parse input: {}", e))
    }

    pub fn parse_torrent_file(&mut self, input: &[u8]) -> Result<TorrentFile> {
        serde_bencode::from_bytes(input).map_err(|e| anyhow!("Failed to parse input: {}", e))
    }
}

pub trait ValueToString {
    fn to_string(&self) -> String;
}

impl ValueToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Bytes(bytes) => {
                format!("{:?}", String::from_utf8_lossy(bytes).to_string())
            }
            Value::Int(int) => int.to_string(),
            Value::List(list) => {
                let mut result = Vec::<String>::new();

                for value in list {
                    result.push(value.to_string());
                }

                format!("[{}]", result.join(", "))
            }
            Value::Dict(dict) => {
                let mut result = Vec::<String>::new();

                for (key, value) in dict {
                    let key_ident = format!("{:?}", String::from_utf8_lossy(key).to_string());

                    result.push(format!("{}: {}", key_ident, value.to_string()));
                }

                result.sort_unstable();

                format!("{{{}}}", result.join(", "))
            }
        }
    }
}
