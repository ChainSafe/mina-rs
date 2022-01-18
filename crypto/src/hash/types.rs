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
use serde::{Deserialize, Serialize};
use wire_type::WireType;

pub(crate) type HashBytes = Box<[u8]>;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, From, PartialOrd)]
pub(crate) struct BaseHash([u8; 32]);

impl From<HashBytes> for BaseHash {
    // TODO: Figure out how to do this without a copy and still have BaseHash serializable
    fn from(b: HashBytes) -> Self {
        let mut o = BaseHash::default();
        o.0.copy_from_slice(&b);
        o
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

//////////////////////////////////////////////////////////////////////////
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType, PartialOrd)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct StateHash(BaseHash);

impl_bs58!(StateHash, version_bytes::STATE_HASH);

impl From<HashBytes> for StateHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

impl Hash for StateHash {
    const PREFIX: &'static HashPrefix = PROTOCOL_STATE;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct LedgerHash(BaseHash);

impl_bs58!(LedgerHash, version_bytes::LEDGER_HASH);

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct CoinBaseHash(BaseHash);

impl_bs58!(CoinBaseHash, 12);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct EpochSeed(BaseHash);

impl_bs58!(EpochSeed, version_bytes::EPOCH_SEED);

impl From<HashBytes> for EpochSeed {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

impl Hash for EpochSeed {
    const PREFIX: &'static HashPrefix = EPOCH_SEED;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct SnarkedLedgerHash(BaseHash);

impl_bs58!(SnarkedLedgerHash, version_bytes::LEDGER_HASH);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct StagedLedgerHash {
    pub non_snark: NonSnarkStagedLedgerHash,
    pub pending_coinbase_hash: CoinBaseHash,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: LedgerHash,
    pub aux_hash: AuxHash,
    pub pending_coinbase_aux: PendingCoinbaseAuxHash,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct AuxHash(pub Vec<u8>);

impl Base58Encodable for AuxHash {
    const VERSION_BYTE: u8 = version_bytes::STAGED_LEDGER_HASH_AUX_HASH;
    const MINA_VERSION_BYTE_COUNT: usize = 0;

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        output.extend(self.0.as_slice());
    }
}

impl From<Vec<u8>> for AuxHash {
    fn from(h: Vec<u8>) -> Self {
        Self(h)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PendingCoinbaseAuxHash(pub Vec<u8>);

impl Base58Encodable for PendingCoinbaseAuxHash {
    const VERSION_BYTE: u8 = version_bytes::STAGED_LEDGER_HASH_PENDING_COINBASE_AUX;
    const MINA_VERSION_BYTE_COUNT: usize = 0;

    fn write_encodable_bytes(&self, output: &mut Vec<u8>) {
        output.extend(self.0.as_slice());
    }
}

impl From<Vec<u8>> for PendingCoinbaseAuxHash {
    fn from(h: Vec<u8>) -> Self {
        Self(h)
    }
}

impl AsRef<[u8]> for AuxHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct VrfOutputHash(BaseHash);

impl_bs58!(VrfOutputHash, version_bytes::VRF_TRUNCATED_OUTPUT);

impl From<HashBytes> for VrfOutputHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

impl Hash for VrfOutputHash {
    const PREFIX: &'static HashPrefix = VRF_OUTPUT;
}

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

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
