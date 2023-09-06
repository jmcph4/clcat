use clap::Parser;
use libp2p::Multiaddr;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[clap(short, long)]
    /// Multiaddress(es) to listen on
    pub listen_address: Vec<Multiaddr>,
    #[clap(short, long)]
    /// Multiaddress(es) to dial
    pub dial_address: Vec<Multiaddr>,
}
