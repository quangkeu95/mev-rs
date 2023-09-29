use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Bundle {}

#[derive(Debug, Deserialize)]
pub struct BundleHash {}

#[derive(Debug, Serialize, Clone)]
pub struct SendBundleResponse {}

#[derive(Debug, Serialize, Clone)]
pub struct SimulateBundleResponse {}

#[derive(Debug, Serialize, Clone)]
pub struct CancelBundleResponse {}
