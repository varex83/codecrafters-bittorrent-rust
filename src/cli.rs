use clap::Parser;
use clap::Subcommand;

/// BitTorrent CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Decodes <string> from the bencode format
    Decode { string: String },
    /// Info of the torrent file located at <PATH>
    Info { path: String },
    /// Peers of the torrent file located at <PATH>
    Peers { path: String },
    /// Handsake with the peer at <IP:PORT>
    Handshake { path: String, ip: String },
}
