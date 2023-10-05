use ethers::signers::Signer;
use ethers::types::H256;
use thiserror::Error;

use crate::constants::{MAX_BLOCK_OFFSET, MAX_BLOCK_RANGE, MAX_NESTING_LEVEL};

use super::types::Bundle;

#[derive(Debug, Default)]
pub struct BundleValidation {
    pub hash: H256,
    pub unmatched: bool,
}

pub fn validate_bundle<S>(
    level: u16,
    bundle: &Bundle,
    current_block_number: u64,
    signer: S,
) -> Result<BundleValidation, BundleValidationError>
where
    S: Signer,
{
    if level > MAX_NESTING_LEVEL {
        return Err(BundleValidationError::BundleTooDeep);
    }
    if bundle.version != "beta-1" && bundle.version != "v0.1" {
        return Err(BundleValidationError::UnsupportedVersion(
            bundle.version.clone(),
        ));
    }

    // validate inclusion
    let min_block = bundle.inclusion.block;
    let max_block =
        if bundle.inclusion.max_block.is_some() && bundle.inclusion.max_block.unwrap() > 0 {
            bundle.inclusion.max_block.unwrap()
        } else {
            bundle.inclusion.block
        };

    if max_block < min_block {
        return Err(BundleValidationError::InvalidInclusion(
            "max block must grearter than or equal to min block".to_string(),
        ));
    }
    if max_block - min_block > MAX_BLOCK_RANGE {
        return Err(BundleValidationError::InvalidInclusion(format!(
            "exceed max block range",
        )));
    }
    if current_block_number >= max_block {
        return Err(BundleValidationError::InvalidInclusion(format!(
            "max block must greater than current block"
        )));
    }
    if min_block > current_block_number + MAX_BLOCK_OFFSET {
        return Err(BundleValidationError::InvalidInclusion(format!(
            "min block must less than current block"
        )));
    }
    if bundle.body.len() == 0 {
        return Err(BundleValidationError::InvalidBundleBodySize);
    }

    let mut txs = 0;
    let mut body_hashes: Vec<H256> = vec![];
    let mut unmatched = false;

    for (i, body) in bundle.body.iter().enumerate() {
        if let Some(tx_hash) = &body.hash {
            // make sure that we have up to one unmatched element and only at the beginning of the body
            if unmatched || i > 0 {
                return Err(BundleValidationError::InvalidBundleBody(
                    "unmatched".to_string(),
                ));
            }
            unmatched = true;
            body_hashes.push(tx_hash.clone());
            if bundle.body.len() == 1 {
                // we have unmatched bundle without anything else
                return Err(BundleValidationError::InvalidBundleBody(
                    "bundle body with single unmatched item".to_string(),
                ));
            }
            txs += 1;
        } else if let Some(bundle_body) = &body.bundle {
        }
    }

    Ok(BundleValidation::default())
}

#[derive(Debug, Error)]
pub enum BundleValidationError {
    #[error("bundle too deep")]
    BundleTooDeep,
    #[error("unsupported bundle version: {0}")]
    UnsupportedVersion(String),
    #[error("invalid inclusion, error: {0}")]
    InvalidInclusion(String),
    #[error("invalid bundle body size")]
    InvalidBundleBodySize,
    #[error("invalid bundle body: {0}")]
    InvalidBundleBody(String),
}
