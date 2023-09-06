use crate::cli::Commands;
use crate::parser::{Parser, ValueToString};
use anyhow::Result;
use clap::Parser as ClapParser;
use cli::Cli;
use sha1::{Digest, Sha1};

mod cli;
mod parser;

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

            let mut hasher = Sha1::default();

            hasher.update(serde_bencode::to_bytes(&result.info)?);

            let res = hasher.finalize();

            println!("Info Hash: {:x}", res);
        }
    }

    Ok(())
}
