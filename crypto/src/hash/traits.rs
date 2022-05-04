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
pub trait Hash {
    const PREFIX: &'static HashPrefix;
}

/// Any internal type that needs to be hashed must implement this trait
/// Since each type should have its own HashType the implementation is generic over the output type
///
/// Typically the implementation need not specify any of the functions as all can be derived from Serialize
///
pub trait Hashable<OutputType>: Sized + Serialize
where
    OutputType: Hash + From<Box<[u8]>>,
{
    fn hash(&self) -> OutputType {
        // this is known to be a valid hash size
        let mut hasher = Blake2bVar::new(BLAKE_HASH_SIZE).unwrap();
        // writing to a hasher can't fail
        to_writer(&mut hasher, self).unwrap();
        OutputType::from(hasher.finalize_boxed())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::hash::prefixes::PROTOCOL_STATE;
    use crate::hash::types::BaseHash;
    use crate::impl_from_for_hash;
    use mina_serialization_types::{impl_strconv_via_json, json::*, v1::*, version_bytes};
    use versioned::*;

    #[derive(Clone, PartialEq, Debug)]
    struct TestHash(BaseHash);
    type TestHashV1Json = HashV1Json<{ version_bytes::STATE_HASH }>;
    impl_from_for_hash!(TestHash, HashV1);
    impl_from_for_generic_with_proxy!(TestHash, HashV1, TestHashV1Json);
    impl_strconv_via_json!(TestHash, TestHashV1Json);

    impl From<Box<[u8]>> for TestHash {
        fn from(b: Box<[u8]>) -> Self {
            Self(BaseHash::from(b))
        }
    }

    impl Hash for TestHash {
        const PREFIX: &'static HashPrefix = PROTOCOL_STATE;
    }

    impl AsRef<[u8]> for TestHash {
        fn as_ref(&self) -> &[u8] {
            &self.0 .0.as_ref()
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
            h,
            TestHash::from_str("3NLXw1spzQFnLEJGQQKVyykTFExSBjLuhfEU32Fez3odCwY3A4Yc").unwrap()
        );
        assert_eq!(
            h.to_string(),
            "3NLXw1spzQFnLEJGQQKVyykTFExSBjLuhfEU32Fez3odCwY3A4Yc"
        );
    }

    #[test]
    fn hash_changes_with_data() {
        let t1 = TestType(123);
        let t2 = TestType(234);
        assert!(t1.hash() != t2.hash())
    }
}
