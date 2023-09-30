use std::{net::SocketAddr, time::Instant};

use jsonrpsee::{
    helpers::MethodResponseResult,
    server::logger::{
        HttpRequest, Logger as JsonrpseeLogger, MethodKind, Params, TransportProtocol,
    },
};
use tracing::info;

#[derive(Clone)]
pub struct RpcServerMetrics {}

impl RpcServerMetrics {
    pub fn new() -> Self {
        Self {}
    }
}

impl JsonrpseeLogger for RpcServerMetrics {
    type Instant = Instant;

    fn on_connect(&self, remote_addr: SocketAddr, request: &HttpRequest, _t: TransportProtocol) {
        info!(
            "[RpcServer::on_connect] remote_address {:?} request {:?}",
            remote_addr, request
        );
    }

    fn on_request(&self, transport: TransportProtocol) -> Self::Instant {
        todo!()
    }

    fn on_call(
        &self,
        method_name: &str,
        params: Params,
        kind: MethodKind,
        transport: TransportProtocol,
    ) {
        todo!()
    }

    fn on_result(
        &self,
        method_name: &str,
        success_or_error: MethodResponseResult,
        started_at: Self::Instant,
        transport: TransportProtocol,
    ) {
        todo!()
    }

    fn on_response(&self, result: &str, started_at: Self::Instant, transport: TransportProtocol) {
        todo!()
    }

    fn on_disconnect(&self, _remote_addr: std::net::SocketAddr, transport: TransportProtocol) {
        todo!()
    }
}
