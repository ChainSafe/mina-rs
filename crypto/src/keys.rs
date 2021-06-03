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
}
