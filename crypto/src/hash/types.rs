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
use crate::hash::Hash;
use mina_serialization_types::{impl_strconv_via_json, json::*, v1::*};
use proof_systems::mina_hasher::{Hashable, ROInput};
use serde::{Deserialize, Serialize};
use versioned::*;

#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    derive_more::From,
    derive_more::Into,
    PartialOrd,
)]
pub(crate) struct BaseHash(pub(crate) [u8; 32]);

impl Hashable for BaseHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_bytes(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl From<Box<[u8]>> for BaseHash {
    // TODO: Figure out how to do this without a copy and still have BaseHash serializable
    fn from(b: Box<[u8]>) -> Self {
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

impl_from_for_newtype!(BaseHash, HashV1);
impl_from_for_newtype!(BaseHash, Hash2V1);

#[macro_export]
macro_rules! impl_from_for_hash {
    ($t:ty, $tv:ty) => {
        impl From<$t> for $tv {
            fn from(t: $t) -> Self {
                t.0.into()
            }
        }

        impl From<$tv> for $t {
            fn from(h: $tv) -> Self {
                let base: BaseHash = h.into();
                Self(base)
            }
        }
    };
}

//////////////////////////////////////////////////////////////////////////
#[derive(Clone, Default, Debug, PartialEq, PartialOrd, derive_more::From, derive_more::Into)]
pub struct StateHash(BaseHash);

impl_from_for_hash!(StateHash, HashV1);
impl_from_for_generic_with_proxy!(StateHash, HashV1, StateHashV1Json);
impl_strconv_via_json!(StateHash, StateHashV1Json);

impl Hashable for StateHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hash for StateHash {
    const PREFIX: &'static HashPrefix = PROTOCOL_STATE;
}

//////////////////////////////////////////////////////////////////////////

#[derive(
    Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From, derive_more::Into,
)]
pub struct LedgerHash(BaseHash);

impl_from_for_hash!(LedgerHash, HashV1);
impl_from_for_generic_with_proxy!(LedgerHash, HashV1, LedgerHashV1Json);
impl_strconv_via_json!(LedgerHash, LedgerHashV1Json);

impl Hashable for LedgerHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct ChainHash(BaseHash);

impl_from_for_hash!(ChainHash, HashV1);
impl_from_for_generic_with_proxy!(ChainHash, HashV1, ChainHashV1Json);
impl_strconv_via_json!(ChainHash, ChainHashV1Json);

//////////////////////////////////////////////////////////////////////////

#[derive(
    Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From, derive_more::Into,
)]
pub struct CoinBaseHash(BaseHash);

impl_from_for_hash!(CoinBaseHash, HashV1);
impl_from_for_hash!(CoinBaseHash, Hash2V1);
impl_from_for_generic_with_proxy!(CoinBaseHash, HashV1, CoinBaseHashV1Json);
impl_strconv_via_json!(CoinBaseHash, CoinBaseHashV1Json);

impl Hashable for CoinBaseHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct EpochSeed(BaseHash);

impl_from_for_hash!(EpochSeed, HashV1);
impl_from_for_generic_with_proxy!(EpochSeed, HashV1, EpochSeedHashV1Json);
impl_strconv_via_json!(EpochSeed, EpochSeedHashV1Json);

impl Hashable for EpochSeed {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl Hash for EpochSeed {
    const PREFIX: &'static HashPrefix = EPOCH_SEED;
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct SnarkedLedgerHash(BaseHash);

impl_from_for_hash!(SnarkedLedgerHash, HashV1);
impl_from_for_generic_with_proxy!(SnarkedLedgerHash, HashV1, LedgerHashV1Json);
impl_strconv_via_json!(SnarkedLedgerHash, LedgerHashV1Json);

impl Hashable for SnarkedLedgerHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StagedLedgerHash {
    pub non_snark: NonSnarkStagedLedgerHash,
    pub pending_coinbase_hash: CoinBaseHash,
}

impl Hashable for StagedLedgerHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.non_snark);
        roi.append_hashable(&self.pending_coinbase_hash);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NonSnarkStagedLedgerHash {
    pub ledger_hash: LedgerHash,
    pub aux_hash: AuxHash,
    pub pending_coinbase_aux: PendingCoinbaseAuxHash,
}

impl Hashable for NonSnarkStagedLedgerHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_hashable(&self.ledger_hash);
        roi.append_hashable(&self.aux_hash);
        roi.append_hashable(&self.pending_coinbase_aux);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(
    Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From, derive_more::Into,
)]
pub struct AuxHash(pub Vec<u8>);

impl_from_for_newtype!(AuxHash, AuxHashJson);
impl_strconv_via_json!(AuxHash, AuxHashJson);

impl Hashable for AuxHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_bytes(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, derive_more::From)]
pub struct PendingCoinbaseAuxHash(pub Vec<u8>);

impl_from_for_newtype!(PendingCoinbaseAuxHash, PendingCoinbaseAuxHashJson);
impl_strconv_via_json!(PendingCoinbaseAuxHash, PendingCoinbaseAuxHashJson);

impl Hashable for PendingCoinbaseAuxHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_bytes(&self.0);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq)]
pub struct VrfOutputHash(BaseHash);

impl_from_for_hash!(VrfOutputHash, HashV1);
impl_from_for_generic_with_proxy!(VrfOutputHash, HashV1, VrfOutputHashV1Json);
impl_strconv_via_json!(VrfOutputHash, VrfOutputHashV1Json);

impl Hash for VrfOutputHash {
    const PREFIX: &'static HashPrefix = VRF_OUTPUT;
}

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

    use std::str::FromStr;

    use mina_serialization_types::{json::*, JsonSerializationType};

    use super::*;

    #[test]
    fn convert_ledger_hash_to_base58() {
        let bytes = [
            182, 175, 122, 248, 93, 142, 245, 54, 161, 170, 103, 111, 123, 128, 48, 218, 84, 208,
            17, 245, 30, 111, 61, 210, 168, 20, 160, 79, 111, 37, 167, 2,
        ];
        let h = LedgerHash(BaseHash(bytes));
        println!("{h}")
    }

    #[test]
    fn ledger_hash_from_base58() {
        let s = "jxV4SS44wHUVrGEucCsfxLisZyUC5QddsiokGH3kz5xm2hJWZ25";
        let h = LedgerHash::from_str(s).unwrap();
        assert_eq!(&h.to_string(), s);
    }

    #[test]
    fn ledger_hash_json_roundtrip() -> anyhow::Result<()> {
        impl JsonSerializationType<'_> for LedgerHash {
            type T = LedgerHashV1Json;
        }

        let s = "jxV4SS44wHUVrGEucCsfxLisZyUC5QddsiokGH3kz5xm2hJWZ25";
        let s_json = format!("\"{s}\"");
        let h = <LedgerHash as JsonSerializationType>::try_from_json(&s_json)?;
        assert_eq!(h.clone().try_into_json()?, s_json);
        let s_json = h.try_into_json()?;
        let str_json: &str = serde_json::from_str(&s_json)?;
        assert_eq!(s, str_json);
        Ok(())
    }

    #[test]
    fn coinbase_hash_from_base58() {
        let s = "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC";
        let h = CoinBaseHash::from_str(s).unwrap();
        assert_eq!(&h.to_string(), s);
    }

    #[test]
    fn coinbase_hash_json_roundtrip() -> anyhow::Result<()> {
        let s = "2n1tLdP2gkifmyVmrmzYXTS4ohPbZPJn6Qq4x55ywrbRWB4543cC";
        let s_json = format!("\"{s}\"");
        let json: CoinBaseHashV1Json = serde_json::from_str(&s_json)?;
        let h: CoinBaseHash = json.into();
        assert_eq!(&h.to_string(), s);
        let json: CoinBaseHashV1Json = h.into();
        assert_eq!(serde_json::to_string(&json)?, s_json);
        Ok(())
    }

    #[test]
    fn epoch_seed_from_base58() {
        let s = "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA";
        let h = EpochSeed::from_str(s).unwrap();
        assert_eq!(&h.to_string(), s);
    }

    #[test]
    fn epoch_seed_hash_json_roundtrip() -> anyhow::Result<()> {
        let s = "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA";
        let s_json = format!("\"{s}\"");
        let json: EpochSeedHashV1Json = serde_json::from_str(&s_json)?;
        let h: EpochSeed = json.into();
        assert_eq!(&h.to_string(), s);
        let json: EpochSeedHashV1Json = h.into();
        assert_eq!(serde_json::to_string(&json)?, s_json);
        Ok(())
    }

    #[test]
    fn snarked_ledger_hash_from_base58() {
        let s = "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee";
        let h = SnarkedLedgerHash::from_str(s).unwrap();
        assert_eq!(&h.to_string(), s);
    }

    #[test]
    fn snarked_ledger_hash_json_roundtrip() -> anyhow::Result<()> {
        let s = "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee";
        let s_json = format!("\"{s}\"");
        let json: LedgerHashV1Json = serde_json::from_str(&s_json)?;
        let h: SnarkedLedgerHash = json.into();
        assert_eq!(&h.to_string(), s);
        let json: LedgerHashV1Json = h.into();
        assert_eq!(serde_json::to_string(&json)?, s_json);
        Ok(())
    }

    #[test]
    fn roundtrip() {
        let bytes = [
            0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00_u8, 0x01, 0x02,
            0x03, 0x04, 0x05, 0x06, 0x07,
        ];
        let h = LedgerHash(BaseHash(bytes));
        assert_eq!(h, LedgerHash::from_str(h.to_string().as_str()).unwrap())
    }
}
