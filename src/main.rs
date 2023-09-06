use crate::cli::Commands;
use crate::hasher::{bytes_to_hex, hash_bytes, hash_bytes_urlencode};
use crate::parser::{Parser, ValueToString};
use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;

mod cli;
mod hasher;
mod parser;
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

            let peers = request.get_peers(t_file.announce).get_peers().unwrap();

            println!("Peers:");

            for peer in peers {
                println!("{}:{}", peer.0, peer.1);
            }
        }
    }

    Ok(())
}
