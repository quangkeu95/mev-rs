use clap::{Args as ClapArgs, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Start MEV share node
    #[command()]
    Node(NodeCommand),
}

#[derive(Debug, ClapArgs)]
pub struct NodeCommand {
    /// HTTP port
    #[arg(short, long, default_value = "8080", env)]
    pub port: u16,
    /// Redis channel name
    #[arg(long, default_value = "hints", env)]
    pub redis_channel_name: String,
    /// Redis endpoint
    #[arg(long, default_value = "redis://localhost:6379", env)]
    pub redis_endpoint: String,
    /// Simulation endpoint
    #[arg(long, num_args(0..), default_value = "http://localhost:8545", env)]
    pub simulation_endpoint: Vec<String>,
}
