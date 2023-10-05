use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use jsonrpsee::core::RpcResult;
use jsonrpsee::server::Server;
use mev_share::rpc::api::MevShareApiServer;
use mev_share::rpc::types::Bundle;
use mev_share::rpc::types::SendBundleResponse;
use mev_share::rpc::{metrics::RpcServerMetrics, server::RpcServer};
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter};

#[cfg(feature = "client")]
use {
    jsonrpsee::core::client::ClientT, jsonrpsee::http_client::HttpClientBuilder,
    jsonrpsee::rpc_params, mev_share::rpc::api::MevShareApiClient,
};

pub fn test_address() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0))
}

// #[ignore]
#[cfg(feature = "client")]
#[tokio::test]
async fn test_rpc_server() {
    init_test_tracing();
    let rpc_server = RpcServer::new();
    let metrics = RpcServerMetrics::new();
    let socket_addr = test_address();

    let server = Server::builder()
        .set_logger(metrics)
        .build(socket_addr)
        .await
        .unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(rpc_server.into_rpc());

    tokio::spawn(handle.stopped());

    let url = format!("http://{}", addr);
    info!("rpc server endpoint: {:?}", url);

    let client = HttpClientBuilder::default().build(&url).unwrap();
    let bundle = rpc_params![Bundle::default()];

    let send_bundle_res = client
        .request::<SendBundleResponse, _>("mev_sendBundle", bundle)
        .await
        .unwrap();

    // info!("send bundle response {:#?}", send_bundle_res);
    assert_eq!(
        send_bundle_res.bundle_hash,
        SendBundleResponse::default().bundle_hash
    );
}

fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();
}
