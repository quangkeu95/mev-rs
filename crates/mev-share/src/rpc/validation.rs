use ethers::signers::Signer;
use ethers::types::H256;
use thiserror::Error;

use crate::{
    constants::{MAX_BLOCK_OFFSET, MAX_BLOCK_RANGE, MAX_BODY_SIZE, MAX_NESTING_LEVEL},
    utils::{decode_signed_tx, merge_inclusion_intervals, merge_privacy_builders},
};

use super::types::{Bundle, BundleInclusion};

#[derive(Debug, Default)]
pub struct BundleValidation {
    pub hash: H256,
    pub txs: u64,
    pub unmatched: bool,
}

pub fn validate_bundle<S>(
    level: u16,
    bundle: &mut Bundle,
    current_block_number: u64,
    signer: S,
) -> Result<BundleValidation, BundleValidationError>
where
    S: Signer + Clone,
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

    let mut txs: u64 = 0;
    let mut body_hashes: Vec<H256> = vec![];
    let mut unmatched = false;
    let bundle_body_len = bundle.body.len();

    for (i, body) in bundle.body.iter_mut().enumerate() {
        if let Some(tx_hash) = &body.hash {
            // make sure that we have up to one unmatched element and only at the beginning of the body
            if unmatched || i > 0 {
                return Err(BundleValidationError::InvalidBundleBody(
                    "unmatched".to_string(),
                ));
            }
            unmatched = true;
            body_hashes.push(tx_hash.clone());
            if bundle_body_len == 1 {
                // we have unmatched bundle without anything else
                return Err(BundleValidationError::InvalidBundleBody(
                    "bundle body with single unmatched item".to_string(),
                ));
            }
            txs += 1;
        } else if let Some(tx_raw) = &body.tx {
            let (signed_tx, sig) = decode_signed_tx(tx_raw)
                .map_err(|e| BundleValidationError::InvalidBundleBody(e.to_string()))?;
            let tx_hash = signed_tx.hash(&sig);
            body_hashes.push(tx_hash);
            txs += 1;
        } else if let Some(inner_bundle) = &mut body.bundle {
            merge_inclusion_intervals(&mut bundle.inclusion, &inner_bundle.inclusion)
                .map_err(|e| BundleValidationError::InvalidInclusion(e.to_string()))?;

            merge_privacy_builders(bundle.privacy.as_mut(), inner_bundle.privacy.as_ref());

            let inner_bundle_validation = validate_bundle(
                level + 1,
                inner_bundle,
                current_block_number,
                signer.clone(),
            )?;
            body_hashes.push(inner_bundle_validation.hash);
            txs += inner_bundle_validation.txs;

            // don't allow unmatched bundles below 1-st level
            if inner_bundle_validation.unmatched {
                return Err(BundleValidationError::InvalidBundleBody(
                    "inner bundle unmatched".to_string(),
                ));
            }
        }
    }

    if txs > MAX_BODY_SIZE {
        return Err(BundleValidationError::InvalidBundleBodySize);
    }
    if body_hashes.len() == 1 {
        // special case of bundle with a single tx
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
