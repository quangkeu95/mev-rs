use super::EthClient;
use anyhow::{anyhow, Result};
use ethers::{
    providers::{JsonRpcClient, Middleware, Provider},
    types::{Block, BlockNumber, H256},
};
use moka::sync::Cache;
use std::time::Duration;

pub const BLOCK_CACHE_DURATION: Duration = Duration::from_secs(5);

pub struct EthCachingClient<P> {
    provider: Provider<P>,
    block_cache: Cache<String, Block<H256>>,
}

impl<P: JsonRpcClient> EthCachingClient<P> {
    pub fn new(provider: Provider<P>) -> Self {
        let block_cache = Cache::builder().time_to_live(BLOCK_CACHE_DURATION).build();
        Self {
            provider,
            block_cache,
        }
    }
}

#[async_trait::async_trait]
impl<P: JsonRpcClient> EthClient for EthCachingClient<P> {
    async fn get_latest_block(&self) -> Result<Block<H256>> {
        if let Some(block) = self.block_cache.get("latest_block") {
            Ok(block.clone())
        } else {
            let block_res = self.provider.get_block(BlockNumber::Latest).await?;

            let block = block_res.ok_or(anyhow!("error fetching latest block"))?;
            self.block_cache
                .insert("latest_block".to_string(), block.clone());
            Ok(block)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[tokio::test]
    async fn get_latest_block_should_cache() {
        let provider = Provider::try_from("https://eth.llamarpc.com").unwrap();

        let eth_caching_client = EthCachingClient::new(provider);

        let current = Instant::now();
        let _block = eth_caching_client.get_latest_block().await.unwrap();
        let duration = current.elapsed();

        // dbg!(duration);

        let current = Instant::now();
        let _block = eth_caching_client.get_latest_block().await.unwrap();
        let cache_duration = current.elapsed();

        // dbg!(cache_duration);

        assert!(cache_duration < duration);
    }
}
