use crate::base_58::MinaBase58;
use serde::{Deserialize, Serialize};
use serde_versions_derive::version;

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct CompressedPoly {
    x: [u8; 32],
    is_odd: bool,
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct PublicKey {
    poly: CompressedPoly,
}

impl PublicKey {
    pub fn new() -> Self {
        PublicKey {
            poly: CompressedPoly {
                x: [0x0; 32],
                is_odd: false,
            },
        }
    }
}

impl MinaBase58 for PublicKey {
    fn version_byte() -> u8 {
        crate::base58_version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED
    }
}

#[cfg(test)]
pub mod tests {

    use super::PublicKey;
    use crate::base_58::MinaBase58;
    use serde_bin_prot::to_writer;

    #[test]
    fn serialize_empty_keypair() {
        let mut buf = Vec::new();
        to_writer(&mut buf, &PublicKey::new()).unwrap();
        println!("{:?}", buf)
    }

    #[test]
    fn from_base58_roundtrip() {
        let s = "B62qonDZEKYULNkfq7WGu1Z881YBRnMSuBGGX5DhnTv26mUyvN99mpo";
        let k = PublicKey::from_base58(s).unwrap();
        assert_eq!(s, k.to_base58().into_string())
    }
}
