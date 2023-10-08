use ethers::types::H256;
use moka::sync::Cache;

pub const BUNDLE_CACHE_SIZE: u64 = 1000;

/// LRU bundle hash caching with limited capacity
#[derive(Debug)]
pub struct BundleCache {
    cache: Cache<H256, ()>,
}

impl BundleCache {
    pub fn new(size: Option<u64>) -> Self {
        let size = size.unwrap_or(BUNDLE_CACHE_SIZE);
        let cache = Cache::builder().max_capacity(size).build();

        Self { cache }
    }

    /// Return boolean indicate whether [BundleCache] has the bundle hash
    pub fn contains(&self, hash: &H256) -> bool {
        self.cache.contains_key(hash)
    }

    /// Add bundle hash to cache
    pub fn add(&mut self, hash: &H256) {
        self.cache.insert(hash.clone(), ())
    }
}
