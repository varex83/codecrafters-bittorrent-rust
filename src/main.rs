use crate::cli::Commands;
use crate::hasher::{bytes_to_hex, hash_bytes, hash_bytes_urlencode};
use crate::parser::{Parser, ValueToString};
use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;
use std::net::Ipv4Addr;
use std::str::FromStr;

mod cli;
mod hasher;
mod parser;
mod peer;
mod requests;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode { string } => {
            let mut parser = Parser::new();

            let result = parser.decode_input(string)?;

            println!("{}", result.to_string())
        }
        Commands::Info { path } => {
            let t_file = Parser::from_path(&path)?;

            println!("Tracker URL: {}", t_file.announce);
            println!("Length: {}", t_file.info.length);

            println!(
                "Info Hash: {}",
                hash_bytes(&serde_bencode::to_bytes(&t_file.info)?)
            );
            println!("Piece Length: {}", t_file.info.piece_length);
            println!("Piece Hashes:");

            for piece in t_file.info.pieces.chunks(20) {
                println!("{}", bytes_to_hex(piece));
            }
        }
        Commands::Peers { path } => {
            let t_file = Parser::from_path(&path)?;

            let mut request = requests::TrackerRequestBuilder::new()
                .info_hash(hash_bytes_urlencode(&serde_bencode::to_bytes(
                    &t_file.info,
                )?))
                .port(6881)
                .uploaded(0)
                .downloaded(0)
                .left(t_file.info.length)
                .compact(1)
                .build();

            let peers = request.execute(t_file.announce).get_peers().unwrap();

            println!("Peers:");

            for peer in peers {
                println!("{}:{}", peer.0, peer.1);
            }
        }
        Commands::Handshake { path, ip } => {
            let t_file = Parser::from_path(&path)?;

            let (ip, port) = ip.split_once(":").unwrap();

            let ip = Ipv4Addr::from_str(ip).unwrap();
            let port = port.parse::<u16>().unwrap();

            let mut peer = peer::Peer::new(ip, port);

            let info_hash = Vec::from(hash_bytes_urlencode(&serde_bencode::to_bytes(
                &t_file.info,
            )?));

            peer.handshake(
                info_hash,
                Vec::from(hash_bytes_urlencode(&b"12345678901234567890"[..])),
            )
            .unwrap();

            let peer_id = peer.get_id().unwrap();

            println!("Info hash: {}", bytes_to_hex(&info_hash));
            println!("Peer ID: {}", bytes_to_hex(&peer_id));
        }
    }

    Ok(())
}
