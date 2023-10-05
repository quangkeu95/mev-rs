use anyhow::Result;
use ethers::types::{Block, H256};

mod caching_client;
pub use caching_client::*;

/// Client to get info from RPC provider
#[async_trait::async_trait]
pub trait EthClient {
    /// Get the most recent block
    async fn get_latest_block(&self) -> Result<Block<H256>>;
}
