use jsonrpsee::proc_macros::rpc;
use jsonrpsee_core::RpcResult;

use super::types::{
    Bundle, BundleHash, CancelBundleResponse, SendBundleResponse, SimulateBundleResponse,
};

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "mev"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "mev"))]
pub trait MevShareApi {
    #[method(name = "sendBundle")]
    async fn send_bundle(&self, bundle: Bundle) -> RpcResult<SendBundleResponse>;
    #[method(name = "simBundle")]
    async fn simulate_bundle(&self, bundle: Bundle) -> RpcResult<SimulateBundleResponse>;
    #[method(name = "cancelBundleByHash")]
    async fn cancel_bundle_by_hash(
        &self,
        bundle_hash: BundleHash,
    ) -> RpcResult<CancelBundleResponse>;
}
