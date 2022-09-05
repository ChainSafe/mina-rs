// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Extensions for [Fp], including json serde, etc.

use ark_ff::BigInteger256;
use derive_more::{From, Into};
use mina_hasher::Fp;
use num::{BigUint, Num};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, From, Into)]
pub struct FpJson(pub Fp);

impl Serialize for FpJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let big256: BigInteger256 = self.0.into();
        let big: BigUint = big256.into();
        serializer.serialize_str(big.to_str_radix(10).as_str())
    }
}

impl<'de> Deserialize<'de> for FpJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let big =
            BigUint::from_str_radix(&s, 10).map_err(<D::Error as serde::de::Error>::custom)?;
        let big256: BigInteger256 = big
            .try_into()
            .map_err(<D::Error as serde::de::Error>::custom)?;
        Ok(Self(big256.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fp_json_serde_roundtrip() -> anyhow::Result<()> {
        let json_str =
            "\"15955265048676495861308121109313805689802008176208259245703497109342504292532\"";
        let obj: FpJson = serde_json::from_str(json_str)?;
        assert_eq!(serde_json::to_string(&obj)?.as_str(), json_str);
        Ok(())
    }
}
