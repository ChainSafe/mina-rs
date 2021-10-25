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

use super::prefixes::*;
use crate::base58::{version_bytes, Base58Encodable};
use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

pub(crate) type HashBytes = Box<[u8]>;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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

//////////////////////////////////////////////////////////////////////////
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct BackendCommonHash([u8; 32]);

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct StateHash(BaseHash);

impl Base58Encodable for StateHash {
    const VERSION_BYTE: u8 = version_bytes::STATE_HASH;
}

impl From<HashBytes> for StateHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

impl AsRef<[u8]> for StateHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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

impl Base58Encodable for LedgerHash {
    const VERSION_BYTE: u8 = version_bytes::LEDGER_HASH;
}

impl From<HashBytes> for LedgerHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct CoinBaseHash(BaseHash);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct EpochSeed(BaseHash);

impl Base58Encodable for EpochSeed {
    const VERSION_BYTE: u8 = version_bytes::EPOCH_SEED;
}

impl From<HashBytes> for EpochSeed {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

impl AsRef<[u8]> for EpochSeed {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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

impl Base58Encodable for SnarkedLedgerHash {
    const VERSION_BYTE: u8 = version_bytes::LEDGER_HASH;
}

impl From<HashBytes> for SnarkedLedgerHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

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
#[wire_type(recurse = 1)]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: LedgerHash,
    pub aux_hash: AuxHash,
    pub pending_coinbase_aux: AuxHash,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 1)]
pub struct AuxHash(Vec<u8>);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct VrfOutputHash(BaseHash);

impl Base58Encodable for VrfOutputHash {
    const VERSION_BYTE: u8 = version_bytes::VRF_TRUNCATED_OUTPUT;
}

impl Hash for VrfOutputHash {
    const PREFIX: &'static HashPrefix = VRF_OUTPUT;
}

impl AsRef<[u8]> for VrfOutputHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<HashBytes> for VrfOutputHash {
    fn from(b: HashBytes) -> Self {
        Self(BaseHash::from(b))
    }
}

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

    use super::{BaseHash, LedgerHash};
    use crate::base58::Base58Encodable;

    #[test]
    fn convert_hash_to_base58() {
        let bytes = [
            182, 175, 122, 248, 93, 142, 245, 54, 161, 170, 103, 111, 123, 128, 48, 218, 84, 208,
            17, 245, 30, 111, 61, 210, 168, 20, 160, 79, 111, 37, 167, 2,
        ];
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
        let bytes = [
            0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02,
            0x03, 0x04, 0x05, 0x06, 0x07,
        ];
        let h = LedgerHash(BaseHash(bytes));
        assert_eq!(
            h.clone(),
            LedgerHash::from_base58(h.to_base58().into_string()).unwrap()
        )
    }
}
