use versioned::Versioned;
use crate::signature::{PublicKey, CompressedCurvePoint, Signature, InnerCurveScalar};
use mina_serialization_types::{v1::{PublicKeyV1, PublicKey2V1, SignatureV1}, signatures::InnerCurveScalar as InnerCurveScalarV1};

impl From<PublicKey> for PublicKeyV1 {
    fn from(t: PublicKey) -> Self {
        Self::new(Versioned::new(
            mina_serialization_types::signatures::CompressedCurvePoint {
                x: t.poly.x,
                is_odd: t.poly.is_odd,
            },
        ))
    }
}
impl From<PublicKeyV1> for PublicKey {
    fn from(t: PublicKeyV1) -> Self {
        Self {
            poly: CompressedCurvePoint {
                x: t.t.t.x,
                is_odd: t.t.t.is_odd,
            },
        }
    }
}

impl From<PublicKey> for PublicKey2V1 {
    fn from(t: PublicKey) -> Self {
        Self::new(Versioned::new(
            Versioned::new(
            mina_serialization_types::signatures::CompressedCurvePoint {
                x: t.poly.x,
                is_odd: t.poly.is_odd,
            },
        )))
    }
}
impl From<PublicKey2V1> for PublicKey {
    fn from(t: PublicKey2V1) -> Self {
        Self {
            poly: CompressedCurvePoint {
                x: t.t.t.t.x,
                is_odd: t.t.t.t.is_odd,
            },
        }
    }
}

impl From<Signature> for SignatureV1 {
    fn from(t: Signature) -> Self {
        Self::new(Versioned::new(
            (t.0.0.into(), t.0.1.into())
        ))
    }
}
impl From<SignatureV1> for Signature {
    fn from(t: SignatureV1) -> Self {
        Self ((t.t.t.0.into(), t.t.t.1.into()))
    }
}

impl From<InnerCurveScalar> for InnerCurveScalarV1 {
    fn from(t: InnerCurveScalar) -> Self {
        Self(t.into())
    }
}
impl From<InnerCurveScalarV1> for InnerCurveScalar {
    fn from(t: InnerCurveScalarV1) -> Self {
        Self (t.0)
    }
}

