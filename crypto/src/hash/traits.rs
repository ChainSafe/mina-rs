// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::prefixes::HashPrefix;
use blake2::digest::VariableOutput;
use blake2::VarBlake2b;
use serde::Serialize;
use serde_bin_prot::to_writer;

const BLAKE_HASH_SIZE: usize = 32;

/// Trait that any internal hash wrapper type must implement
/// This defines the prefix that is added to the data prior to it being hashed
pub trait MinaHash
where
    Self: From<Box<[u8]>>,
{
    fn prefix() -> &'static HashPrefix;
}

/// Any internal type that needs to be hashed must implement this trait
/// Since each type should have its own HashType the implementation is generic over the output type
///
/// Typically the implementation need not specify any of the functions as all can be derived from Serialize
///
pub trait MinaHashable<OutputType>: Sized + Serialize
where
    OutputType: MinaHash,
{
    fn hash(&self) -> OutputType {
        // this is known to be a valid hash size
        let mut hasher = VarBlake2b::new(BLAKE_HASH_SIZE).unwrap();
        // writing to a hasher can't fail
        to_writer(&mut hasher, self).unwrap();
        OutputType::from(hasher.finalize_boxed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base58::{version_bytes, MinaBase58};
    use crate::hash::prefixes::PROTOCOL_STATE;
    use crate::hash::types::{BaseHash, HashBytes};

    #[derive(Serialize, PartialEq, Debug)]
    struct TestHash(BaseHash);

    impl From<HashBytes> for TestHash {
        fn from(b: HashBytes) -> Self {
            Self(BaseHash::from(b))
        }
    }

    impl MinaBase58 for TestHash {
        const VERSION_BYTE: u8 = version_bytes::STATE_HASH;
    }

    impl MinaHash for TestHash {
        fn prefix() -> &'static HashPrefix {
            PROTOCOL_STATE
        }
    }

    #[derive(Serialize)]
    struct TestType(i32);
    impl MinaHashable<TestHash> for TestType {}

    #[test]
    fn can_hash_new_type() {
        let t = TestType(123);
        let h = t.hash();
        assert_eq!(
            h.to_base58().into_string(),
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
