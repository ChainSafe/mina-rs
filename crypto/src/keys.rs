use serde::{Deserialize, Serialize};
use serde_versions_derive::version;

use crate::base_58::MinaBase58;

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

impl AsRef<[u8]> for PublicKey {
	// TODO: This is wrong
	fn as_ref(&self) -> &[u8] {
		&self.poly.x
	}
}

impl MinaBase58 for PublicKey {
	fn version_byte() -> u8 { crate::base58_version_bytes::NON_ZERO_CURVE_POINT_COMPRESSED }
}

impl ToString for PublicKey {
	fn to_string(&self) -> String {
		self.clone().to_base58().into_string()
	}
}



#[cfg(test)]
pub mod tests {

    use super::PublicKey;
    use serde_bin_prot::to_writer;

    #[test]
    fn serialize_empty_keypair() {
        let mut buf = Vec::new();
        to_writer(&mut buf, &PublicKey::new()).unwrap();
        println!("{:?}", buf)
    }

    #[test]
    fn string_print_empty_keypair() {
        println!("{}", PublicKey::new().to_string())
    }

}
