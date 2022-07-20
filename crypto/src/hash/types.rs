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

use mina_serialization_types::{impl_strconv_via_json, json::*, v1::*};
use proof_systems::{
    mina_hasher::{Fp, Hashable, ROInput},
    o1_utils::{field_helpers::FieldHelpersError, FieldHelpers},
};
use sha2::{Digest, Sha256};
use versioned::*;

#[derive(Clone, Debug, Default, PartialEq, derive_more::From, derive_more::Into)]
pub(crate) struct BaseHash(pub(crate) [u8; 32]);

impl Hashable for BaseHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_field(self.try_into().expect("Failed to convert Hash into Fp"));
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl From<&[u8]> for BaseHash {
    fn from(b: &[u8]) -> Self {
        let mut o = BaseHash::default();
        o.0.copy_from_slice(b);
        o
    }
}

impl From<&Fp> for BaseHash {
    fn from(i: &Fp) -> Self {
        i.to_bytes().as_slice().into()
    }
}

impl TryFrom<&BaseHash> for Fp {
    type Error = FieldHelpersError;

    fn try_from(i: &BaseHash) -> Result<Self, Self::Error> {
        Fp::from_bytes(i.0.as_slice())
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
#[derive(Clone, Debug, Default, PartialEq, derive_more::From, derive_more::Into)]
pub struct StateHash(BaseHash);

impl_from_for_hash!(StateHash, HashV1);
impl_from_for_generic_with_proxy!(StateHash, HashV1, StateHashV1Json);
impl_strconv_via_json!(StateHash, StateHashV1Json);

impl From<[u8; 32]> for StateHash {
    fn from(v: [u8; 32]) -> Self {
        Self(BaseHash(v))
    }
}

impl From<StateHash> for [u8; 32] {
    fn from(v: StateHash) -> Self {
        v.0 .0
    }
}

impl From<&Fp> for StateHash {
    fn from(i: &Fp) -> Self {
        let base: BaseHash = i.into();
        base.into()
    }
}

impl TryFrom<&StateHash> for Fp {
    type Error = FieldHelpersError;

    fn try_from(i: &StateHash) -> Result<Self, Self::Error> {
        (&i.0).try_into()
    }
}

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

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, derive_more::From, derive_more::Into)]
pub struct LedgerHash(BaseHash);

impl_from_for_hash!(LedgerHash, HashV1);
impl_from_for_generic_with_proxy!(LedgerHash, HashV1, LedgerHashV1Json);
impl_strconv_via_json!(LedgerHash, LedgerHashV1Json);

impl From<&Fp> for LedgerHash {
    fn from(i: &Fp) -> Self {
        let base: BaseHash = i.into();
        base.into()
    }
}

impl TryFrom<&LedgerHash> for Fp {
    type Error = FieldHelpersError;

    fn try_from(i: &LedgerHash) -> Result<Self, Self::Error> {
        (&i.0).try_into()
    }
}

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

impl Hashable for ChainHash {
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

impl From<[u8; 32]> for ChainHash {
    fn from(v: [u8; 32]) -> Self {
        Self(BaseHash(v))
    }
}

impl From<ChainHash> for [u8; 32] {
    fn from(v: ChainHash) -> Self {
        v.0 .0
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, derive_more::From, derive_more::Into)]
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

impl NonSnarkStagedLedgerHash {
    pub fn digest(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let ledger_hash_bytes: Vec<u8> = self.ledger_hash.0 .0.iter().rev().cloned().collect();
        hasher.update(&ledger_hash_bytes);
        hasher.update(&self.aux_hash.0);
        hasher.update(&self.pending_coinbase_aux.0);
        hasher.finalize().to_vec()
    }
}

impl Hashable for NonSnarkStagedLedgerHash {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi.append_bytes(&self.digest());
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

#[derive(Clone, Default, Debug, PartialEq, derive_more::From, derive_more::Into)]
pub struct AuxHash(pub Vec<u8>);

impl_from_for_newtype!(AuxHash, AuxHashJson);
impl_strconv_via_json!(AuxHash, AuxHashJson);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq, derive_more::From)]
pub struct PendingCoinbaseAuxHash(pub Vec<u8>);

impl_from_for_newtype!(PendingCoinbaseAuxHash, PendingCoinbaseAuxHashJson);
impl_strconv_via_json!(PendingCoinbaseAuxHash, PendingCoinbaseAuxHashJson);

//////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug, PartialEq)]
pub struct VrfOutputHash(BaseHash);

impl_from_for_hash!(VrfOutputHash, HashV1);
impl_from_for_generic_with_proxy!(VrfOutputHash, HashV1, VrfOutputHashV1Json);
impl_strconv_via_json!(VrfOutputHash, VrfOutputHashV1Json);

//////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod test {

    use std::str::FromStr;

    use mina_serialization_types::{json::*, JsonSerializationType};

    use super::*;

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
        let h = LedgerHash::from_str(s).unwrap();
        assert_eq!(&h.to_string(), s);
    }

    #[test]
    fn snarked_ledger_hash_json_roundtrip() -> anyhow::Result<()> {
        let s = "jx7buQVWFLsXTtzRgSxbYcT8EYLS8KCZbLrfDcJxMtyy4thw2Ee";
        let s_json = format!("\"{s}\"");
        let json: LedgerHashV1Json = serde_json::from_str(&s_json)?;
        let h: LedgerHash = json.into();
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
