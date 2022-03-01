// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::prefixes::HashPrefix;
use bin_prot::to_writer;
use blake2::digest::VariableOutput;
use blake2::Blake2bVar;
use serde::Serialize;

const BLAKE_HASH_SIZE: usize = 32;

/// Trait that any internal hash wrapper type must implement
/// This defines the prefix that is added to the data prior to it being hashed
pub trait Hash
where
    Self: From<Box<[u8]>>,
{
    const PREFIX: &'static HashPrefix;
}

/// Any internal type that needs to be hashed must implement this trait
/// Since each type should have its own HashType the implementation is generic over the output type
///
/// Typically the implementation need not specify any of the functions as all can be derived from Serialize
///
pub trait Hashable<OutputType>: Sized + Serialize
where
    OutputType: Hash,
{
    fn hash(&self) -> OutputType {
        // this is known to be a valid hash size
        let mut hasher = Blake2bVar::new(BLAKE_HASH_SIZE).unwrap();
        // writing to a hasher can't fail
        to_writer(&mut hasher, self).unwrap();
        OutputType::from(hasher.finalize_boxed())
    }
}

/// Trait that converts given generic value into either field(s) or bits(s)
/// and append to mutable [mina_signer::ROInput], this trait should be implemented for [mina_signer::ROInput]
pub trait RandomOraclePartialInput<T> {
    fn append(&mut self, value: &T);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base58::{version_bytes, Base58Encodable};
    use crate::binprot::BinProtEncodable;
    use crate::hash::prefixes::PROTOCOL_STATE;
    use crate::hash::types::{BaseHash, HashBytes};
    use crate::impl_bs58_for_binprot;
    use serde::Deserialize;
    use wire_type::WireType;

    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
    struct TestHash(BaseHash);

    impl BinProtEncodable for TestHash {
        const PREALLOCATE_BUFFER_BYTES: usize = 64;
    }

    impl_bs58_for_binprot!(TestHash, version_bytes::STATE_HASH);

    impl From<HashBytes> for TestHash {
        fn from(b: HashBytes) -> Self {
            Self(BaseHash::from(b))
        }
    }

    impl Hash for TestHash {
        const PREFIX: &'static HashPrefix = PROTOCOL_STATE;
    }

    impl AsRef<[u8]> for TestHash {
        fn as_ref(&self) -> &[u8] {
            &self.0.as_ref()
        }
    }

    #[derive(Serialize)]
    struct TestType(i32);
    impl Hashable<TestHash> for TestType {}

    #[test]
    fn can_hash_new_type() {
        let t = TestType(123);
        let h = t.hash();
        assert_eq!(
            h.to_base58_string(),
            "Zbx5bAfiyj8yPh8nhXEW3et2TEbnZvEPrShQxTaJaLX3cvPPZV"
        )
    }

    #[test]
    fn hash_changes_with_data() {
        let t1 = TestType(123);
        let t2 = TestType(234);
        assert!(t1.hash() != t2.hash())
    }
}
