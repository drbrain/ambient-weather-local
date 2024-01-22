use clap::Parser;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address to bind to
    #[arg(default_value_t = default_bind_addr(), long)]
    pub address: SocketAddr,

    /// Maximum reports to save
    #[arg(default_value_t = 20, long)]
    pub max_reports: usize,
}

fn default_bind_addr() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9111)
}
