use versioned::Versioned;
use crate::signature::{PublicKey, CompressedCurvePoint, Signature, InnerCurveScalar};
use mina_serialization_types::{v1::{PublicKeyV1, PublicKey2V1, SignatureV1, StagedLedgerHashV1, NonSnarkStagedLedgerHashV1}, signatures::InnerCurveScalar as InnerCurveScalarV1};
use crate::hash::{StagedLedgerHash, NonSnarkStagedLedgerHash};

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

impl From<NonSnarkStagedLedgerHash> for NonSnarkStagedLedgerHashV1 {
    fn from(t: NonSnarkStagedLedgerHash) -> Self {
        NonSnarkStagedLedgerHashV1::new(
            mina_serialization_types::blockchain_state::NonSnarkStagedLedgerHash {
                ledger_hash: t.ledger_hash.into_inner().into(),
                aux_hash: t.aux_hash.0.into(),
                pending_coinbase_aux: t.pending_coinbase_aux.0.into(),
            }
        )
    }
}
impl From<NonSnarkStagedLedgerHashV1> for NonSnarkStagedLedgerHash {
    fn from(t: NonSnarkStagedLedgerHashV1) -> Self {
        Self {
            ledger_hash: t.t.ledger_hash.into(),
            aux_hash: t.t.aux_hash.t.into(),
            pending_coinbase_aux: t.t.pending_coinbase_aux.t.into(),
        }
    }
}

impl From<StagedLedgerHash> for StagedLedgerHashV1 {
    fn from(t: StagedLedgerHash) -> Self {
        StagedLedgerHashV1::new(
            Versioned::new(Versioned::new(
            mina_serialization_types::blockchain_state::StagedLedgerHash {
                non_snark: t.non_snark.into(),
                pending_coinbase_hash: Versioned::new(t.pending_coinbase_hash.into_inner().into()),
            }
        )))
    }
}
impl From<StagedLedgerHashV1> for StagedLedgerHash {
    fn from(t: StagedLedgerHashV1) -> Self {
        Self {
            non_snark: t.t.t.t.non_snark.into(),
            pending_coinbase_hash: t.t.t.t.pending_coinbase_hash.t.into(),
        }
    }
}
