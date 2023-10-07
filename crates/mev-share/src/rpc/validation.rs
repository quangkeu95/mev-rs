use std::collections::HashSet;

use ethers::types::H256;
use ethers::utils::keccak256;
use ethers::{signers::Signer, types::Bytes};
use thiserror::Error;

use crate::{
    constants::{MAX_BLOCK_OFFSET, MAX_BLOCK_RANGE, MAX_BODY_SIZE, MAX_NESTING_LEVEL},
    utils::{decode_signed_tx, merge_inclusion_intervals, merge_privacy_builders},
};

use super::types::{Bundle, BundleInclusion, Hint};

#[derive(Debug, Default)]
pub struct BundleValidation {
    pub hash: H256,
    pub txs: u64,
    pub unmatched: bool,
}

impl BundleValidation {
    pub fn new(hash: H256, txs: u64, unmatched: bool) -> Self {
        Self {
            hash,
            txs,
            unmatched,
        }
    }
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
    let max_block = bundle.inclusion.max_block.unwrap_or(bundle.inclusion.block);

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

    let hash = if body_hashes.len() == 1 {
        // special case of bundle with a single tx
        body_hashes[0]
    } else {
        let concat_hashes = body_hashes.iter().fold(Bytes::new(), |acc, item| {
            let item: Bytes = item.clone().to_fixed_bytes().into();
            [acc, item].concat().into()
        });

        let h = keccak256(concat_hashes);
        H256::from(h)
    };

    // validate validity
    if unmatched && bundle.validity.refund.len() > 0 {
        // refunds should be empty for unmatched bundles
        return Err(BundleValidationError::InvalidBundleConstraints);
    }

    let total_refund_config_percent = bundle
        .validity
        .refund_config
        .iter()
        .map(|refund_config| {
            if refund_config.percent > 100 {
                Err(BundleValidationError::InvalidBundleConstraints)
            } else {
                Ok(refund_config.percent)
            }
        })
        .sum::<Result<u64, BundleValidationError>>()?;

    if total_refund_config_percent > 100 {
        return Err(BundleValidationError::InvalidBundleConstraints);
    }

    let mut user_body_pos: HashSet<u64> = HashSet::new();
    let mut total_percent = 0u64;

    for refund_constraint in bundle.validity.refund.iter() {
        if refund_constraint.body_idx >= bundle.body.len() as u64 {
            return Err(BundleValidationError::InvalidBundleConstraints);
        }
        if user_body_pos.contains(&refund_constraint.body_idx) {
            return Err(BundleValidationError::InvalidBundleConstraints);
        }
        user_body_pos.insert(refund_constraint.body_idx);

        if refund_constraint.percent > 100 {
            return Err(BundleValidationError::InvalidBundleConstraints);
        }
        total_percent += refund_constraint.percent;
    }

    if total_percent > 100 {
        return Err(BundleValidationError::InvalidBundleConstraints);
    }

    if unmatched && bundle.privacy.is_some() && !bundle.privacy.as_ref().unwrap().hints.is_empty() {
        return Err(BundleValidationError::InvalidBundlePrivacy);
    }

    if let Some(privacy) = &mut bundle.privacy {
        if privacy.hints.len() > 0 {
            privacy.hints.push(Hint::Hash);
        }
        if privacy.want_refund < 0 || privacy.want_refund > 100 {
            return Err(BundleValidationError::InvalidBundlePrivacy);
        }
    }

    // clean metadata
    bundle.metadata = None;

    Ok(BundleValidation::new(hash, txs, unmatched))
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
    #[error("invalid bundle constraints")]
    InvalidBundleConstraints,
    #[error("invalid bundle privacy")]
    InvalidBundlePrivacy,
}
