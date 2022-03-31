// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Signatures and public key types

use crate::field_and_curve_elements::FieldElement;
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// An EC point stored in compressed form containing only the x coordinate and one extra bit
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CompressedCurvePoint {
    /// The x coordinate of the EC point
    pub x: FieldElement,
    /// If the point is odd (or even)
    pub is_odd: bool,
}

/// Wrapper type for field element denoting it is on the curves scalar field
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct InnerCurveScalar(pub FieldElement);

/// Public key (v1)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKeyV1(pub Versioned<Versioned<CompressedCurvePoint, 1>, 1>);

/// Public key (v1) with an extra version byte
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicKey2V1(pub Versioned<PublicKeyV1, 1>); // with an extra version wrapper

/// Signature (v1)
pub type SignatureV1 = Versioned<Versioned<(FieldElement, InnerCurveScalar), 1>, 1>;

mod conversions {
    use super::{CompressedCurvePoint, PublicKey2V1, PublicKeyV1};
    use mina_signer::{BaseField, CompressedPubKey};
    use o1_utils::field_helpers::FieldHelpers;
    use versioned::Versioned;

    impl From<PublicKeyV1> for CompressedPubKey {
        fn from(t: PublicKeyV1) -> Self {
            CompressedPubKey {
                x: BaseField::from_bytes(&t.0.t.t.x).unwrap(),
                is_odd: t.0.t.t.is_odd,
            }
        }
    }
    impl From<CompressedPubKey> for PublicKeyV1 {
        fn from(t: CompressedPubKey) -> Self {
            PublicKeyV1(Versioned::new(Versioned::new(CompressedCurvePoint {
                x: t.x.to_bytes().as_slice().try_into().unwrap(),
                is_odd: t.is_odd,
            })))
        }
    }

    impl From<PublicKey2V1> for CompressedPubKey {
        fn from(t: PublicKey2V1) -> Self {
            t.0.t.into()
        }
    }
    impl From<CompressedPubKey> for PublicKey2V1 {
        fn from(t: CompressedPubKey) -> Self {
            PublicKey2V1(Versioned::new(t.into()))
        }
    }
}
