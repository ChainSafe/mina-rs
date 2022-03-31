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
    use super::{PublicKeyV1, PublicKey2V1, CompressedCurvePoint};
    use versioned::Versioned;
    use o1_utils::field_helpers::FieldHelpers;
    use mina_signer::{BaseField, CompressedPubKey};

    impl Into<PublicKeyV1> for CompressedPubKey {
        fn into(self) -> PublicKeyV1 {
            PublicKeyV1(Versioned::new(Versioned::new(CompressedCurvePoint {
                x: self.x.to_bytes().as_slice().try_into().unwrap(),
                is_odd: self.is_odd
            })))
        }
    }
    impl Into<CompressedPubKey> for PublicKeyV1 {
        fn into(self) -> CompressedPubKey {
            CompressedPubKey {
                x: BaseField::from_bytes(&self.0.t.t.x).unwrap(),
                is_odd: self.0.t.t.is_odd,
            }
        }
    }

    impl Into<PublicKey2V1> for CompressedPubKey {
        fn into(self) -> PublicKey2V1 {
            PublicKey2V1(Versioned::new(self.into()))
        }
    }
    impl Into<CompressedPubKey> for PublicKey2V1 {
        fn into(self) -> CompressedPubKey {
            self.0.t.into()
        }
    }
}
