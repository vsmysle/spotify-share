use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    GetToken {
        #[arg(long, required(true))]
        cache_directory: std::path::PathBuf,
    },
    RunServer {
        #[arg(long, env, default_value_t = std::net::Ipv4Addr::new(0, 0, 0, 0))]
        ip: std::net::Ipv4Addr,
        #[arg(long)]
        port: u16,
        #[arg(long, required(true))]
        cache_directory: std::path::PathBuf,
    },
}
