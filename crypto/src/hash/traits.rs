// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0
//
use super::prefixes::HashPrefix;
use digest::Digest;
use generic_array::GenericArray;
use serde::Serialize;
use serde_bin_prot::to_writer;
use sha2::Sha256;
use std::io::Write;

/// Trait that any internal hash wrapper type must implement
/// This defines the prefix that is added to the data prior to it being hashed
pub trait MinaHash<Hasher = Sha256>
where
    Self: From<GenericArray<u8, Hasher::OutputSize>>,
    Hasher: Digest,
{
    fn prefix() -> &'static HashPrefix;
}

/// Any internal type that needs to be hashed must implement this trait
/// Since each type should have its own HashType the implementation is generic over the output type
///
/// Typically the implementation need not specify any of the functions as all can be derived from Serialize
///
/// Implementation can also specify a different hash algorithm (default: Sha256)
/// Is generic over output size as long as OutputType supports conversion from a GenericArray of that size
pub trait MinaHashable<OutputType, Hasher = Sha256>: Sized + Serialize
where
    OutputType: MinaHash<Hasher>,
    Hasher: Digest,
{
    fn hash(&self) -> OutputType {
        let mut buf = Vec::<u8>::new();
        // write the prefix bytes (can unwrap as writing to a vec should always be safe)
        buf.write_all(OutputType::prefix()).unwrap();
        // write the data bytes (can unwrap as writing to a vec should always be safe)
        to_writer(&mut buf, self).unwrap();
        // compute the hash
        let hash_bytes = Hasher::digest(&buf);
        OutputType::from(hash_bytes)
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
        fn version_byte() -> u8 {
            version_bytes::STATE_HASH
        }
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
            "YHWQQZSuu7LGyXydNDKcT47vkbotjcHMeWeHHZj8K6z7EkVphQ"
        )
    }

    #[test]
    fn hash_changes_with_data() {
        let t1 = TestType(123);
        let t2 = TestType(234);
        assert!(t1.hash() != t2.hash())
    }
}
