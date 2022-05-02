// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::proof_messages::{
    ProofMessageWithDegreeBound, ProofMessageWithoutDegreeBoundList,
};
use crate::types::*;
use mina_serialization_types::v1::*;

impl From<ProofMessageWithDegreeBound> for ProofMessageWithDegreeBoundV1 {
    fn from(t: ProofMessageWithDegreeBound) -> Self {
        mina_serialization_types::proof_messages::ProofMessageWithDegreeBound::from(t).into()
    }
}
impl From<ProofMessageWithDegreeBoundV1> for ProofMessageWithDegreeBound {
    fn from(t: ProofMessageWithDegreeBoundV1) -> Self {
        t.t.into()
    }
}

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

impl From<ProofMessages> for ProofMessagesV1 {
    fn from(t: ProofMessages) -> Self {
        mina_serialization_types::proof_messages::ProofMessages::from(t).into()
    }
}
impl From<ProofMessagesV1> for ProofMessages {
    fn from(t: ProofMessagesV1) -> Self {
        t.t.into()
    }
}

impl From<ProofEvaluations> for ProofEvaluationsV1 {
    fn from(t: ProofEvaluations) -> Self {
        mina_serialization_types::proof_evaluations::ProofEvaluations::from(t).into()
    }
}
impl From<ProofEvaluationsV1> for ProofEvaluations {
    fn from(t: ProofEvaluationsV1) -> Self {
        t.t.into()
    }
}

impl From<OpeningProof> for OpeningProofV1 {
    fn from(t: OpeningProof) -> Self {
        mina_serialization_types::opening_proof::OpeningProof::from(t).into()
    }
}
impl From<OpeningProofV1> for OpeningProof {
    fn from(t: OpeningProofV1) -> Self {
        t.t.into()
    }
}

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

impl From<Proof> for ProofV1 {
    fn from(t: Proof) -> Self {
        mina_serialization_types::protocol_state_proof::Proof::from(t).into()
    }
}
impl From<ProofV1> for Proof {
    fn from(t: ProofV1) -> Self {
        t.t.t.into()
    }
}

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

impl From<PairingBased> for PairingBasedV1 {
    fn from(t: PairingBased) -> Self {
        mina_serialization_types::protocol_state_proof::PairingBased::from(t).into()
    }
}
impl From<PairingBasedV1> for PairingBased {
    fn from(t: PairingBasedV1) -> Self {
        t.t.into()
    }
}

impl From<ProofStatePairingBased> for ProofStatePairingBasedV1 {
    fn from(t: ProofStatePairingBased) -> Self {
        mina_serialization_types::protocol_state_proof::ProofStatePairingBased::from(t).into()
    }
}
impl From<ProofStatePairingBasedV1> for ProofStatePairingBased {
    fn from(t: ProofStatePairingBasedV1) -> Self {
        t.t.into()
    }
}

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

impl From<Plonk> for PlonkV1 {
    fn from(t: Plonk) -> Self {
        mina_serialization_types::protocol_state_proof::Plonk::from(t).into()
    }
}
impl From<PlonkV1> for Plonk {
    fn from(t: PlonkV1) -> Self {
        t.t.into()
    }
}

impl From<ProofStateDeferredValues> for ProofStateDeferredValuesV1 {
    fn from(t: ProofStateDeferredValues) -> Self {
        mina_serialization_types::protocol_state_proof::ProofStateDeferredValues {
            plonk: t.plonk.into(),
            combined_inner_product: t.combined_inner_product.into(),
            b: t.b.into(),
            xi: t.xi.into(),
            bulletproof_challenges: t.bulletproof_challenges.into(),
            which_branch: t.which_branch.0.into(),
        }
        .into()
    }
}
impl From<ProofStateDeferredValuesV1> for ProofStateDeferredValues {
    fn from(t: ProofStateDeferredValuesV1) -> Self {
        Self {
            plonk: t.t.plonk.into(),
            combined_inner_product: t.t.combined_inner_product.into(),
            b: t.t.b.into(),
            xi: t.t.xi.into(),
            bulletproof_challenges: t.t.bulletproof_challenges.into(),
            which_branch: t.t.which_branch.t.into(),
        }
    }
}

impl From<ProofState> for ProofStateV1 {
    fn from(t: ProofState) -> Self {
        mina_serialization_types::protocol_state_proof::ProofState::from(t).into()
    }
}
impl From<ProofStateV1> for ProofState {
    fn from(t: ProofStateV1) -> Self {
        t.t.into()
    }
}

impl From<ProofStatement> for ProofStatementV1 {
    fn from(t: ProofStatement) -> Self {
        mina_serialization_types::protocol_state_proof::ProofStatement::from(t).into()
    }
}
impl From<ProofStatementV1> for ProofStatement {
    fn from(t: ProofStatementV1) -> Self {
        t.t.t.into()
    }
}

impl From<ProtocolStateProof> for ProtocolStateProofV1 {
    fn from(t: ProtocolStateProof) -> Self {
        mina_serialization_types::protocol_state_proof::ProtocolStateProof::from(t).into()
    }
}
impl From<ProtocolStateProofV1> for ProtocolStateProof {
    fn from(t: ProtocolStateProofV1) -> Self {
        t.t.t.t.t.into()
    }
}
