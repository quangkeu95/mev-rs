use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::rpc::{api::MevShareApiServer, metrics::RpcServerMetrics, server::RpcServer};

use super::NodeCommand;
use jsonrpsee::server::Server;
use tracing::info;

pub async fn run(args: NodeCommand) -> anyhow::Result<()> {
    // connect to redis

    let redis_client = redis::Client::open(args.redis_endpoint)?;
    info!("connecting to redis");

    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), args.port);
    let metrics = RpcServerMetrics::new();
    let rpc_server = RpcServer::new();

    let server = Server::builder()
        .set_logger(metrics)
        .build(socket_addr)
        .await?;
    let addr = server.local_addr()?;
    let handle = server.start(rpc_server.into_rpc());
    info!("RPC server is served at {:?}", addr);

    handle.stopped().await;
    Ok(())
}
