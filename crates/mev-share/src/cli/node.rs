use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::rpc::{metrics::RpcServerMetrics, server::RpcServer};

use super::NodeCommand;
use tracing::info;

pub async fn run(args: NodeCommand) -> anyhow::Result<()> {
    // connect to redis

    let redis_client = redis::Client::open(args.redis_endpoint)?;
    info!("connecting to redis");

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.port);
    let metrics = RpcServerMetrics::new();
    let rpc_server = RpcServer::new();
    rpc_server.run(socket_addr, metrics).await
}
