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

use crate::base58_version_bytes;
use crate::base_58::MinaBase58;
use serde::{Deserialize, Serialize};

pub use sha2::Sha256 as DefaultHasher;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
struct BaseHash([u8; 32]);

//////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StateHash(BaseHash);

impl MinaBase58 for StateHash {
    fn version_byte() -> u8 {
        base58_version_bytes::STATE_HASH
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LedgerHash(BaseHash);

impl MinaBase58 for LedgerHash {
    fn version_byte() -> u8 {
        base58_version_bytes::LEDGER_HASH
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EpochSeed(BaseHash);

impl MinaBase58 for EpochSeed {
    fn version_byte() -> u8 {
        base58_version_bytes::EPOCH_SEED
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SnarkedLedgerHash(BaseHash);

impl MinaBase58 for SnarkedLedgerHash {
    fn version_byte() -> u8 {
        base58_version_bytes::LEDGER_HASH
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StagedLedgerHash(BaseHash);

impl MinaBase58 for StagedLedgerHash {
    fn version_byte() -> u8 {
        base58_version_bytes::STAGED_LEDGER_HASH_AUX_HASH
    }
}

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

    use super::{BaseHash, LedgerHash};
    use crate::base_58::MinaBase58;

    #[test]
    fn convert_hash_to_base58() {
        let bytes = [
            182, 175, 122, 248, 93, 142, 245, 54, 161, 170, 103, 111, 123, 128, 48, 218, 84,
            208, 17, 245, 30, 111, 61, 210, 168, 20, 160, 79, 111, 37, 167, 2,
        ];
        let h = LedgerHash(BaseHash(bytes));
        println!("{}", h.to_base58().into_string())
    }

    #[test]
    fn ledger_hash_from_base58() {
        let s = "AjUewFHZA8MjLKQYQPB81ktdNyJ34GKitNFfoWTuZREYMog84Q";
        let h = LedgerHash::from_base58(s).unwrap();
        assert_eq!(h.to_base58().into_string(), s);
    }

    #[test]
    fn roundtrip() {
        let bytes = [
            0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03,
            0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        ];
        let h = LedgerHash(BaseHash(bytes));
        assert_eq!(
            h.clone(),
            LedgerHash::from_base58(h.to_base58().into_string()).unwrap()
        )
    }
}
