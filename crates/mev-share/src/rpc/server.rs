use alloy_primitives::B256;
use async_trait::async_trait;
use jsonrpsee::server::{Server, ServerHandle};
use jsonrpsee_core::RpcResult;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::info;

use super::{
    api::MevShareApiServer,
    metrics::RpcServerMetrics,
    types::{Bundle, CancelBundleResponse, SendBundleResponse, SimulateBundleResponse},
};

#[derive(Debug)]
pub struct RpcServer {}

impl RpcServer {
    pub fn new() -> Self {
        Self {}
    }

    // pub async fn run(self, socket_addr: SocketAddr, metrics: RpcServerMetrics) -> ServerHandle {
    //     let server = Server::builder()
    //         .set_logger(metrics)
    //         .build(socket_addr)
    //         .await?;

    //     let handle = server.start(self.into_rpc());
    //     info!("rpc server started at {}", socket_addr);

    //     handle.stopped().await;
    //     Ok(())
    // }
}

#[async_trait]
impl MevShareApiServer for RpcServer {
    async fn send_bundle(&self, bundle: Bundle) -> RpcResult<SendBundleResponse> {
        Ok(SendBundleResponse::default())
    }
    async fn simulate_bundle(&self, bundle: Bundle) -> RpcResult<SimulateBundleResponse> {
        Ok(SimulateBundleResponse::default())
    }
    async fn cancel_bundle_by_hash(&self, bundle_hash: B256) -> RpcResult<CancelBundleResponse> {
        Ok(CancelBundleResponse::default())
    }
}

// pub fn start_rpc_server(rpc_server: RpcServer, socket_addr: SocketAddr, metrics: RpcServerMetrics)
