use ethers::{
    types::{
        transaction::eip2718::{TypedTransaction, TypedTransactionError},
        Signature,
    },
    utils::rlp::Rlp,
};

/// Decode raw bytes into [TypedTransaction]
pub fn decode_signed_tx(
    signed_tx_raw: &[u8],
) -> Result<(TypedTransaction, Signature), TypedTransactionError> {
    let signed_tx_rlp = Rlp::new(signed_tx_raw);

    TypedTransaction::decode_signed(&signed_tx_rlp)
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use ethers::types::{Bytes, H160};

    use super::decode_signed_tx;

    #[test]
    fn can_decode_signed_tx() {
        let signed_tx_raw= Bytes::from_str("0x02f86b0180843b9aca00852ecc889a0082520894c87037874aed04e51c29f582394217a0a2b89d808080c080a0a463985c616dd8ee17d7ef9112af4e6e06a27b071525b42182fe7b0b5c8b4925a00af5ca177ffef2ff28449292505d41be578bebb77110dfc09361d2fb56998260").unwrap();

        let signed_tx = decode_signed_tx(&signed_tx_raw.0).unwrap();
        let eip1559_tx = signed_tx.0.as_eip1559_ref().unwrap();
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
