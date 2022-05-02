// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::proof_messages::{
    ProofMessageWithDegreeBound, ProofMessageWithoutDegreeBoundList,
};
use crate::types::*;
use mina_serialization_types::v1::*;
use versioned::*;

impl_from_for_versioned_with_proxy!(
    ProofMessageWithDegreeBound,
    mina_serialization_types::proof_messages::ProofMessageWithDegreeBound,
    ProofMessageWithDegreeBoundV1
);

impl From<ProofMessageWithoutDegreeBoundList> for ProofMessageWithoutDegreeBoundListV1 {
    fn from(t: ProofMessageWithoutDegreeBoundList) -> Self {
        t.0.into_iter().map(Into::into).collect::<Vec<_>>().into()
    }
}
impl From<ProofMessageWithoutDegreeBoundListV1> for ProofMessageWithoutDegreeBoundList {
    fn from(t: ProofMessageWithoutDegreeBoundListV1) -> Self {
        Self(t.t.t.into_iter().map(Into::into).collect())
    }
}

impl_from_for_versioned_with_proxy!(
    ProofMessages,
    mina_serialization_types::proof_messages::ProofMessages,
    ProofMessagesV1
);

impl_from_for_versioned_with_proxy!(
    ProofEvaluations,
    mina_serialization_types::proof_evaluations::ProofEvaluations,
    ProofEvaluationsV1
);

impl_from_for_versioned_with_proxy!(
    OpeningProof,
    mina_serialization_types::opening_proof::OpeningProof,
    OpeningProofV1
);

impl From<ProofOpenings> for ProofOpeningsV1 {
    fn from(t: ProofOpenings) -> Self {
        mina_serialization_types::protocol_state_proof::ProofOpenings {
            proof: t.proof.into(),
            evals: (t.evals.0.into(), t.evals.1.into()),
        }
        .into()
    }
}
impl From<ProofOpeningsV1> for ProofOpenings {
    fn from(t: ProofOpeningsV1) -> Self {
        Self {
            proof: t.t.proof.into(),
            evals: (t.t.evals.0.into(), t.t.evals.1.into()),
        }
    }
}

impl_from_for_versioned_with_proxy!(
    Proof,
    mina_serialization_types::protocol_state_proof::Proof,
    ProofV1
);

impl From<PrevXHat> for PrevXHatV1 {
    fn from(t: PrevXHat) -> Self {
        PrevXHatV1::new(t.0.into())
    }
}
impl From<PrevXHatV1> for PrevXHat {
    fn from(t: PrevXHatV1) -> Self {
        Self(t.t.into())
    }
}

impl From<PrevEvals> for PrevEvalsV1 {
    fn from(t: PrevEvals) -> Self {
        PrevEvalsV1::new((t.0 .0.into(), t.0 .1.into()))
    }
}
impl From<PrevEvalsV1> for PrevEvals {
    fn from(t: PrevEvalsV1) -> Self {
        Self((t.t.0.into(), t.t.1.into()))
    }
}

impl_from_for_versioned_with_proxy!(
    PairingBased,
    mina_serialization_types::protocol_state_proof::PairingBased,
    PairingBasedV1
);

impl_from_for_versioned_with_proxy!(
    ProofStatePairingBased,
    mina_serialization_types::protocol_state_proof::ProofStatePairingBased,
    ProofStatePairingBasedV1
);

impl From<SpongeDigestBeforeEvaluations> for SpongeDigestBeforeEvaluationsV1 {
    fn from(t: SpongeDigestBeforeEvaluations) -> Self {
        mina_serialization_types::protocol_state_proof::SpongeDigestBeforeEvaluations((
            t.0 .0.into(),
            t.0 .1.into(),
            t.0 .2.into(),
            t.0 .3.into(),
            (),
        ))
        .into()
    }
}
impl From<SpongeDigestBeforeEvaluationsV1> for SpongeDigestBeforeEvaluations {
    fn from(t: SpongeDigestBeforeEvaluationsV1) -> Self {
        Self((
            t.t.t.0 .0.t.into(),
            t.t.t.0 .1.t.into(),
            t.t.t.0 .2.t.into(),
            t.t.t.0 .3.t.into(),
            (),
        ))
    }
}

impl From<ShiftedValue> for ShiftedValueV1 {
    fn from(t: ShiftedValue) -> Self {
        use mina_serialization_types::protocol_state_proof::ShiftedValue as SV;
        match t {
            ShiftedValue::ShiftedValue(v) => Self::new(SV::ShiftedValue(v.0)),
        }
    }
}
impl From<ShiftedValueV1> for ShiftedValue {
    fn from(t: ShiftedValueV1) -> Self {
        use mina_serialization_types::protocol_state_proof::ShiftedValue as SV;
        match t.t {
            SV::ShiftedValue(v) => Self::ShiftedValue(v.into()),
            _ => unimplemented!(),
        }
    }
}

impl_from_for_versioned_with_proxy!(
    Plonk,
    mina_serialization_types::protocol_state_proof::Plonk,
    PlonkV1
);

impl_from_for_versioned_with_proxy!(
    ProofStateDeferredValues,
    mina_serialization_types::protocol_state_proof::ProofStateDeferredValues,
    ProofStateDeferredValuesV1
);

impl_from_for_versioned_with_proxy!(
    ProofState,
    mina_serialization_types::protocol_state_proof::ProofState,
    ProofStateV1
);

impl_from_for_versioned_with_proxy!(
    ProofStatement,
    mina_serialization_types::protocol_state_proof::ProofStatement,
    ProofStatementV1
);

impl_from_for_versioned_with_proxy!(
    ProtocolStateProof,
    mina_serialization_types::protocol_state_proof::ProtocolStateProof,
    ProtocolStateProofV1
);
