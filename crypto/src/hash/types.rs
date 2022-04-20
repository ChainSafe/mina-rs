// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! Hash and Hasher types reused throughout
//!
//! When converted to human readable forms, hashes in Mina use the Bitcoin Base58Check encoding
//! see <https://github.com/MinaProtocol/mina/blob/f88edb440e321114e26f7691e599adab30ce16cd/src/lib/base58_check/README.md>
//!
//! Depending on the type of hash a different byte prefix is used in the human readable form
//!

use super::prefixes::*;
use crate::base58::{version_bytes, Base58Encodable};
use crate::hash::Hash;
use crate::impl_bs58;
use derive_more::From;
use mina_serialization_types::v1::HashV1;
use serde::{Deserialize, Serialize};
use versioned::impl_from_for_newtype;

pub(crate) type HashBytes = Box<[u8]>;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, From, PartialOrd)]
pub(crate) struct BaseHash([u8; 32]);

impl From<HashBytes> for BaseHash {
    // TODO: Figure out how to do this without a copy and still have BaseHash serializable
    fn from(b: HashBytes) -> Self {
        let b: &[u8] = &b;
        b.into()
    }
}

impl<'a> From<&'a [u8]> for BaseHash {
    fn from(b: &'a [u8]) -> Self {
        let mut o = BaseHash::default();
        o.0.copy_from_slice(b);
        o
    }
}

impl AsRef<[u8]> for BaseHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for BaseHash {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl_from_for_newtype!(BaseHash, HashV1);

impl From<HashV1> for BaseHash {
    fn from(h: HashV1) -> Self {
        Self(h.t)
    }
}

#[macro_export]
macro_rules! impl_from_for_hash {
    ($t:ty, $tv:ty) => {
        impl From<$tv> for $t {
            fn from(h: $tv) -> Self {
                let base: BaseHash = h.into();
                Self(base)
            }
        }
    };
}

//////////////////////////////////////////////////////////////////////////
#[derive(
    Clone, Default, Debug, PartialEq, Serialize, Deserialize, PartialOrd, derive_more::From,
)]
pub struct StateHash(BaseHash);

impl_bs58!(StateHash, version_bytes::STATE_HASH);
impl_from_for_hash!(StateHash, HashBytes);
impl_from_for_hash!(StateHash, HashV1);
impl_from_for_newtype!(StateHash, HashV1);

impl Hash for StateHash {
    const PREFIX: &'static HashPrefix = PROTOCOL_STATE;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From)]
pub struct LedgerHash(BaseHash);

impl_bs58!(LedgerHash, version_bytes::LEDGER_HASH);
impl_from_for_hash!(LedgerHash, HashBytes);
impl_from_for_hash!(LedgerHash, HashV1);
impl_from_for_newtype!(LedgerHash, HashV1);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From)]
pub struct ChainHash(BaseHash);

impl_bs58!(ChainHash, version_bytes::LEDGER_HASH);
impl_from_for_hash!(ChainHash, HashBytes);
impl_from_for_hash!(ChainHash, HashV1);
impl_from_for_newtype!(ChainHash, HashV1);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoinBaseHash(BaseHash);

impl_bs58!(CoinBaseHash, 12);
impl_from_for_hash!(CoinBaseHash, HashBytes);
impl_from_for_hash!(CoinBaseHash, HashV1);
impl_from_for_newtype!(CoinBaseHash, HashV1);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpochSeed(BaseHash);

impl_bs58!(EpochSeed, version_bytes::EPOCH_SEED);
impl_from_for_hash!(EpochSeed, HashBytes);
impl_from_for_hash!(EpochSeed, HashV1);
impl_from_for_newtype!(EpochSeed, HashV1);

impl Hash for EpochSeed {
    const PREFIX: &'static HashPrefix = EPOCH_SEED;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnarkedLedgerHash(BaseHash);

impl_bs58!(SnarkedLedgerHash, version_bytes::LEDGER_HASH);
impl_from_for_hash!(SnarkedLedgerHash, HashBytes);
impl_from_for_hash!(SnarkedLedgerHash, HashV1);
impl_from_for_newtype!(SnarkedLedgerHash, HashV1);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StagedLedgerHash {
    pub non_snark: NonSnarkStagedLedgerHash,
    pub pending_coinbase_hash: CoinBaseHash,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: LedgerHash,
    pub aux_hash: AuxHash,
    pub pending_coinbase_aux: PendingCoinbaseAuxHash,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From)]
#[from(forward)]
pub struct AuxHash(pub Vec<u8>);

impl Base58Encodable for AuxHash {
    const VERSION_BYTE: u8 = version_bytes::STAGED_LEDGER_HASH_AUX_HASH;
    const MINA_VERSION_BYTE_COUNT: usize = 0;

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        output.extend(self.0.as_slice());
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From)]
#[from(forward)]
pub struct PendingCoinbaseAuxHash(pub Vec<u8>);

impl Base58Encodable for PendingCoinbaseAuxHash {
    const VERSION_BYTE: u8 = version_bytes::STAGED_LEDGER_HASH_PENDING_COINBASE_AUX;
    const MINA_VERSION_BYTE_COUNT: usize = 0;

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        output.extend(self.0.as_slice());
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VrfOutputHash(BaseHash);

impl_bs58!(VrfOutputHash, version_bytes::VRF_TRUNCATED_OUTPUT);
impl_from_for_hash!(VrfOutputHash, HashBytes);
impl_from_for_hash!(VrfOutputHash, HashV1);
impl_from_for_newtype!(VrfOutputHash, HashV1);

impl Hash for VrfOutputHash {
    const PREFIX: &'static HashPrefix = VRF_OUTPUT;
}

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

    use mina_serialization_types::common::LedgerHashV1Json;

    use super::*;

    #[test]
    fn convert_ledger_hash_to_base58() {
        let bytes = [
            182, 175, 122, 248, 93, 142, 245, 54, 161, 170, 103, 111, 123, 128, 48, 218, 84, 208,
            17, 245, 30, 111, 61, 210, 168, 20, 160, 79, 111, 37, 167, 2,
        ];
        let h = LedgerHash(BaseHash(bytes));
        println!("{}", h.to_base58_string())
    }

    #[test]
    fn ledger_hash_from_base58() {
        let s = "jxV4SS44wHUVrGEucCsfxLisZyUC5QddsiokGH3kz5xm2hJWZ25";
        let h = LedgerHash::from_base58(s).unwrap();
        assert_eq!(h.to_base58_string(), s);
    }

    #[test]
    fn ledger_hash_json() -> anyhow::Result<()> {
        let s = "jxV4SS44wHUVrGEucCsfxLisZyUC5QddsiokGH3kz5xm2hJWZ25";
        let s_json = format!("\"{s}\"");
        let json: LedgerHashV1Json = serde_json::from_str(&s_json)?;
        let (v1,): (HashV1,) = json.into();
        let h: LedgerHash = v1.into();
        assert_eq!(h.to_base58_string(), s);
        let v1: HashV1 = h.into();
        let json: LedgerHashV1Json = v1.into();
        assert_eq!(serde_json::to_string(&json)?, s_json);
        Ok(())
    }

    #[test]
    fn coinbase_hash_from_base58() {
        let s = "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC";
        let h = CoinBaseHash::from_base58(s).unwrap();
        assert_eq!(h.to_base58_string(), s);
    }

    #[test]
    fn roundtrip() {
        let bytes = [
            0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02,
            0x03, 0x04, 0x05, 0x06, 0x07,
        ];
        let h = LedgerHash(BaseHash(bytes));
        assert_eq!(
            h.clone(),
            LedgerHash::from_base58(h.to_base58_string()).unwrap()
        )
    }
}
