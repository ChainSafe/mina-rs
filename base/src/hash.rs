// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Hash and Hasher types reused throughout
//! 
//! When converted to human readable forms, hashes in Mina use the Bitcoin Base58Check encoding
//! see https://github.com/MinaProtocol/mina/blob/f88edb440e321114e26f7691e599adab30ce16cd/src/lib/base58_check/README.md
//!
//! Depending on the type of hash a different byte prefix is used in the human readable form
//!

use serde::{Deserialize, Serialize};
use mina_base58::{MinaBase58};

pub use sha2::Sha256 as DefaultHasher;


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
struct BaseHash(
	#[serde(with = "big_arrays")]
	[u8; 33]
);

// cannot use derive Default for arrays > 32 elements
impl Default for BaseHash {
	fn default() -> Self { 
		Self([0_u8; 33])
	}
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct StateHash(BaseHash);

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct LedgerHash(BaseHash);

impl AsRef<[u8]> for LedgerHash {
	fn as_ref(&self) -> &[u8] {
		&self.0.0
	}
}

impl AsMut<[u8]> for LedgerHash {
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.0.0
	}
}

impl MinaBase58 for LedgerHash {
	fn version_byte() -> u8 { mina_base58::version_bytes::LEDGER_HASH }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct EpochSeed(BaseHash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct SnarkedLedgerHash(BaseHash);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Hash)]
pub struct StagedLedgerHash(BaseHash);

// Need this to support serde serialization and deserialization of 
// arrays > 32 elements. Will refactor this to its own crate in future
mod big_arrays {
    use std::{convert::TryInto, marker::PhantomData};

    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeTuple,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
        data: &[T; N],
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let mut s = ser.serialize_tuple(N)?;
        for item in data {
            s.serialize_element(item)?;
        }
        s.end()
    }

    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(&format!("an array of length {}", N))
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // can be optimized using MaybeUninit
            let mut data = Vec::with_capacity(N);
            for _ in 0..N {
                match (seq.next_element())? {
                    Some(val) => data.push(val),
                    None => return Err(serde::de::Error::invalid_length(N, &self)),
                }
            }
            match data.try_into() {
                Ok(arr) => Ok(arr),
                Err(_) => unreachable!(),
            }
        }
    }
    pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
    }
}


#[cfg(test)]
pub mod test {

	
	use super::{BaseHash, LedgerHash};
	use mina_base58::MinaBase58;

	#[test]
	fn convert_hash_to_base58() {
		let bytes = [1_u8, 182, 175, 122, 248, 93, 142, 245, 54, 161, 170, 103, 111, 123, 128, 48, 218, 84, 208, 17, 245, 30, 111, 61, 210, 168, 20, 160, 79, 111, 37, 167, 2];
		let h = LedgerHash(BaseHash(bytes));
		println!("{}", h.to_base58().into_string())
	}

	#[test]
	fn ledger_hash_from_base58() {
		let s = "jxV4SS44wHUVrGEucCsfxLisZyUC5QddsiokGH3kz5xm2hJWZ25";
		let h = LedgerHash::from_base58(s).unwrap();
		assert_eq!(h.to_base58().into_string(), s);
	}

	#[test]
	fn roundtrip() {
		let bytes = [0x00_u8, 0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,];
		let h = LedgerHash(BaseHash(bytes));
		assert_eq!(h.clone(), LedgerHash::from_base58(h.to_base58().into_string()).unwrap())
	}

}
