use anyhow::anyhow;
use std::collections::HashSet;

use ethers::{
    types::{
        transaction::eip2718::{TypedTransaction, TypedTransactionError},
        Signature,
    },
    utils::rlp::Rlp,
};

use crate::rpc::types::{BundleInclusion, BundlePrivacy};

/// Decode raw bytes into [TypedTransaction]
pub fn decode_signed_tx(
    signed_tx_raw: &[u8],
) -> Result<(TypedTransaction, Signature), TypedTransactionError> {
    let signed_tx_rlp = Rlp::new(signed_tx_raw);

    TypedTransaction::decode_signed(&signed_tx_rlp)
}

/// Return the [BundleInclusion] contains the value of overlap between the top level and inner [BundleInclusion], if there is no overlap, return None.
pub fn merge_inclusion_intervals(
    top_level_inclusion: &mut BundleInclusion,
    inner_inclusion: &BundleInclusion,
) -> anyhow::Result<()> {
    let top_level_max_block = top_level_inclusion.max_block.unwrap_or_default();
    let inner_max_block = inner_inclusion.max_block.unwrap_or_default();

    if top_level_max_block < inner_inclusion.block || inner_max_block < top_level_inclusion.block {
        return Err(anyhow!("no overlap between inclusions"));
    }
    if top_level_inclusion.block < inner_inclusion.block {
        top_level_inclusion.block = inner_inclusion.block;
    }
    if top_level_max_block > inner_max_block {
        top_level_inclusion.max_block = inner_inclusion.max_block;
    }
    Ok(())
}

/// Return the [BundlePrivacy] contains the value of overlap between the top level and inner [BundlePrivacy]'s builders.
pub fn merge_privacy_builders(
    top_level_privacy: Option<&mut BundlePrivacy>,
    inner_privacy: Option<&BundlePrivacy>,
) {
    if top_level_privacy.is_none() {
        return;
    }
    if inner_privacy.is_none() {
        let top_level_privacy = top_level_privacy.unwrap();
        top_level_privacy.builders = vec![];
        return;
    }

    let top_level_privacy = top_level_privacy.unwrap();

    let top_level_builders = top_level_privacy
        .builders
        .iter()
        .cloned()
        .collect::<HashSet<String>>();

    let inner_builders = inner_privacy
        .unwrap()
        .builders
        .iter()
        .cloned()
        .collect::<HashSet<String>>();

    let builders_intersection = top_level_builders
        .intersection(&inner_builders)
        .cloned()
        .collect::<Vec<String>>();

    top_level_privacy.builders = builders_intersection;
}

// /// Return the intersection of
// fn intersect()

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use ethers::types::{Bytes, H160};

    use super::decode_signed_tx;

    #[test]
    fn can_decode_signed_tx() {
        let signed_tx_raw= Bytes::from_str("0xf8578001825208808862b48d94c41aa714840102030426a0b0359c4b83b06eefabe42e58bc16d7ae87425235fb9ef16f6767b9c24161b5dca0411a61888f6230432bb69597630c8e24865b9be873b403a365144e4a8b2a9b1c").unwrap();

        let (signed_tx, signature) = decode_signed_tx(&signed_tx_raw.0).unwrap();
        // dbg!(&signed_tx);
        // dbg!(&signed_tx.hash(&signature));

        let eip1559_tx = signed_tx.as_eip1559_ref().unwrap();
        assert_eq!(
            eip1559_tx.from.unwrap(),
            H160::from_str("0xc87037874aed04e51c29f582394217a0a2b89d80").unwrap()
        );

        assert_eq!(
            eip1559_tx.to,
            Some(
                H160::from_str("0xc87037874aed04e51c29f582394217a0a2b89d80")
                    .unwrap()
                    .into()
            )
        );
    }
}
