// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module containing the components of a protocol state proof

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use serde::{Deserialize, Serialize};
use versioned::Versioned;

use crate::v1::OpeningProofV1;
use crate::v1::ProofEvaluationsV1;
use crate::v1::ProofMessagesV1;
use crate::v1::{BigInt256, CharV1, Hex64V1};

use crate::v1::{
    BulletproofChallengeTuple18V1, BulletproofChallengesV1, BulletproofPreChallengeV1,
    ProofStateBulletproofChallengesV1, ScalarChallengeVector2V1,
};

use crate::field_and_curve_elements::{FiniteECPoint, FiniteECPointVecV1};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// SNARK proof of the protocol state at some point in time
pub struct ProtocolStateProof {
    pub statement: ProofStatementV1,
    pub prev_evals: PrevEvalsV1,
    pub prev_x_hat: PrevXHatV1,
    pub proof: ProofV1,
}

pub type ProtocolStateProofV1 =
    Versioned<Versioned<Versioned<Versioned<ProtocolStateProof, 1>, 1>, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStatement {
    pub proof_state: ProofStateV1,
    pub pass_through: PairingBasedV1,
}

pub type ProofStatementV1 = Versioned<Versioned<ProofStatement, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofState {
    pub deferred_values: ProofStateDeferredValuesV1,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluationsV1,
    pub me_only: ProofStatePairingBasedV1,
}

pub type ProofStateV1 = Versioned<ProofState, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStateDeferredValues {
    pub plonk: PlonkV1,
    pub combined_inner_product: ShiftedValueV1,
    pub b: ShiftedValueV1,
    pub xi: BulletproofPreChallengeV1,
    pub bulletproof_challenges: BulletproofChallengeTuple18V1,
    pub which_branch: CharV1,
}

pub type ProofStateDeferredValuesV1 = Versioned<ProofStateDeferredValues, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Plonk {
    pub alpha: BulletproofPreChallengeV1,
    pub beta: ScalarChallengeVector2V1,
    pub gamma: ScalarChallengeVector2V1,
    pub zeta: BulletproofPreChallengeV1,
}

pub type PlonkV1 = Versioned<Plonk, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum ShiftedValue {
    ShiftedValue(BigInt256),
}

pub type ShiftedValueV1 = Versioned<ShiftedValue, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SpongeDigestBeforeEvaluations(pub (Hex64V1, Hex64V1, Hex64V1, Hex64V1, ()));

pub type SpongeDigestBeforeEvaluationsV1 =
    Versioned<Versioned<SpongeDigestBeforeEvaluations, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofStatePairingBased {
    pub sg: FiniteECPoint,
    pub old_bulletproof_challenges: ProofStateBulletproofChallengesV1,
}

pub type ProofStatePairingBasedV1 = Versioned<ProofStatePairingBased, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PairingBased {
    pub app_state: (),
    pub sg: FiniteECPointVecV1,
    pub old_bulletproof_challenges: BulletproofChallengesV1,
}

pub type PairingBasedV1 = Versioned<PairingBased, 1>;

pub type PrevEvalsV1 = Versioned<(ProofEvaluationsV1, ProofEvaluationsV1), 1>;

pub type PrevXHatV1 = Versioned<FiniteECPoint, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Proof {
    pub messages: ProofMessagesV1,
    pub openings: ProofOpeningsV1,
}

pub type ProofV1 = Versioned<Versioned<Proof, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofOpenings {
    pub proof: OpeningProofV1,
    pub evals: (ProofEvaluationsV1, ProofEvaluationsV1),
}

pub type ProofOpeningsV1 = Versioned<ProofOpenings, 1>;

/// SNARK proof of the protocol state at some point in time
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolStateProofJson {}

impl From<ProtocolStateProofJson> for ProtocolStateProofV1 {
    fn from(t: ProtocolStateProofJson) -> Self {
        let t: ProtocolStateProof = t.into();
        t.into()
    }
}

impl From<ProtocolStateProofV1> for ProtocolStateProofJson {
    fn from(t: ProtocolStateProofV1) -> Self {
        t.t.t.t.t.into()
    }
}

impl From<ProtocolStateProofJson> for ProtocolStateProof {
    fn from(_t: ProtocolStateProofJson) -> Self {
        unimplemented!()
    }
}

impl From<ProtocolStateProof> for ProtocolStateProofJson {
    fn from(_t: ProtocolStateProof) -> Self {
        unimplemented!()
    }
}
