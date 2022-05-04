// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signatures and public key types

use crate::{
    field_and_curve_elements::{FieldElement, InnerCurveScalar},
    impl_strconv_via_json,
};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_signer::CompressedPubKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use versioned::Versioned;

/// An EC point stored in compressed form containing only the x coordinate and one extra bit
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompressedCurvePoint {
    /// The x coordinate of the EC point
    pub x: FieldElement,
    /// If the point is odd (or even)
    pub is_odd: bool,
}

/// An EC point stored in compressed form containing only the x coordinate and one extra bit (json)
#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(CompressedCurvePoint)]
pub struct PublicKeyJson {
    /// The x coordinate of the EC point
    pub x: FieldElement,
    /// If the point is odd (or even)
    pub is_odd: bool,
}

impl_strconv_via_json!(CompressedCurvePoint, PublicKeyJson);

impl Serialize for PublicKeyJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pk: CompressedCurvePoint = self.clone().into();
        let pk: CompressedPubKey = pk.into();
        let s = pk.into_address();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for PublicKeyJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let pk =
            CompressedPubKey::from_address(&s).map_err(<D::Error as serde::de::Error>::custom)?;
        let pk: CompressedCurvePoint = pk.into();
        Ok(pk.into())
    }
}

/// Public key (v1)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKeyV1(pub Versioned<Versioned<CompressedCurvePoint, 1>, 1>);

/// Public key (v1) with an extra version byte
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKey2V1(pub Versioned<PublicKeyV1, 1>); // with an extra version wrapper

/// Signature (v1)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignatureV1(pub Versioned<Versioned<(FieldElement, InnerCurveScalar), 1>, 1>);

mod conversions {
    use super::{CompressedCurvePoint, PublicKey2V1, PublicKeyJson, PublicKeyV1, SignatureV1};
    use proof_systems::mina_signer::{BaseField, CompressedPubKey, ScalarField, Signature};
    use proof_systems::o1_utils::field_helpers::FieldHelpers;

    impl From<CompressedCurvePoint> for CompressedPubKey {
        fn from(t: CompressedCurvePoint) -> Self {
            CompressedPubKey {
                // This unwrap is safe as a PublicKeyV1 always has 32 bytes of data and from_bytes does not check if it is on curve
                x: BaseField::from_bytes(&t.x)
                    .expect("Wrong number of bytes encountered when converting to BaseField"),
                is_odd: t.is_odd,
            }
        }
    }
    impl From<CompressedPubKey> for CompressedCurvePoint {
        fn from(t: CompressedPubKey) -> Self {
            CompressedCurvePoint {
                // This unwrap of a slice conversion is safe as a CompressedPubKey always has 32 bytes of data which the exact length of
                // FieldElement
                x: t.x
                    .to_bytes()
                    .as_slice()
                    .try_into()
                    .expect("Wrong number of bytes encountered when converting to FieldElement"),
                is_odd: t.is_odd,
            }
        }
    }

    impl From<PublicKeyV1> for CompressedPubKey {
        fn from(t: PublicKeyV1) -> Self {
            let (t,): (CompressedCurvePoint,) = t.0.into();
            t.into()
        }
    }
    impl From<CompressedPubKey> for PublicKeyV1 {
        fn from(t: CompressedPubKey) -> Self {
            let t: CompressedCurvePoint = t.into();
            PublicKeyV1(t.into())
        }
    }

    impl From<PublicKeyV1> for PublicKeyJson {
        fn from(t: PublicKeyV1) -> Self {
            let (t,): (CompressedCurvePoint,) = t.0.into();
            t.into()
        }
    }
    impl From<PublicKeyJson> for PublicKeyV1 {
        fn from(t: PublicKeyJson) -> Self {
            let t: CompressedCurvePoint = t.into();
            PublicKeyV1(t.into())
        }
    }

    impl From<PublicKey2V1> for CompressedPubKey {
        fn from(t: PublicKey2V1) -> Self {
            t.0.t.into()
        }
    }
    impl From<CompressedPubKey> for PublicKey2V1 {
        fn from(t: CompressedPubKey) -> Self {
            let pk: PublicKeyV1 = t.into();
            Self(pk.into())
        }
    }

    impl From<SignatureV1> for Signature {
        fn from(t: SignatureV1) -> Self {
            Signature {
                // This unwrap is safe as a SignatureV1 always has 32 bytes of data and from_bytes does not check if it is on curve
                rx: BaseField::from_bytes(&t.0.t.t.0)
                    .expect("Wrong number of bytes encountered when converting to BaseField"),
                s: ScalarField::from_bytes(&t.0.t.t.1)
                    .expect("Wrong number of bytes encountered when converting to ScalarField"),
            }
        }
    }
    impl From<Signature> for SignatureV1 {
        fn from(t: Signature) -> Self {
            SignatureV1(
                (
                    // This unwrap of a slice conversion is safe as a CompressedPubKey always has 32 bytes of data which the exact length of
                    // FieldElement
                    t.rx.to_bytes().as_slice().try_into().expect(
                        "Wrong number of bytes encountered when converting to FieldElement",
                    ),
                    t.s.to_bytes().as_slice().try_into().expect(
                        "Wrong number of bytes encountered when converting to FieldElement",
                    ),
                )
                    .into(),
            )
        }
    }
}
