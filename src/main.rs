use crate::cli::Commands;
use crate::parser::{Parser, ValueToString};
use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;
use crate::hasher::hash_bytes;

mod cli;
mod parser;
mod hasher;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Decode { string } => {
            let mut parser = Parser::new();

            let result = parser.decode_input(string)?;

            println!("{}", result.to_string())
        }
        Commands::Info { path } => {
            let mut parser = Parser::new();

            let file_content = std::fs::read(path)?;

            let result = parser.parse_torrent_file(&file_content)?;

            println!("Tracker URL: {}", result.announce);
            println!("Length: {}", result.info.length);

            println!("Info Hash: {}", hash_bytes(&serde_bencode::to_bytes(&result.info)?));
            println!("Piece Length: {}", result.info.piece_length);
            println!("Piece Hashes:");

            for piece in result.info.pieces.chunks(20) {
                println!("{}", hash_bytes(piece));
            }

        }
    }

    Ok(())
}
