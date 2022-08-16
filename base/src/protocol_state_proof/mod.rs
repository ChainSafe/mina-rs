// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module containing the components of a protocol state proof

// Much of this crate will be replaced by arkworks and 01-proof-systems types
// so full documentation will not be included
#![allow(missing_docs)]

use crate::numbers::{BigInt256, Char, Hex64};
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::*;
use versioned::*;

pub mod proof_messages;
pub use proof_messages::ProofMessages;

pub mod proof_evaluations;
pub use proof_evaluations::ProofEvaluations;

pub mod opening_proof;
pub use opening_proof::OpeningProof;

pub mod bulletproof_challenges;
pub use bulletproof_challenges::{
    BulletproofChallenge, BulletproofChallengeTuple17, BulletproofChallengeTuple18,
    BulletproofChallenges, BulletproofPreChallenge, ProofStateBulletproofChallenges,
    ScalarChallengeVector2,
};

pub mod field_and_curve_elements;
pub use field_and_curve_elements::{
    ECPoint, ECPointVec, FieldElement, FieldElementVec, FiniteECPoint, FiniteECPointPair,
    FiniteECPointPairVec, FiniteECPointVec,
};

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProtocolStateProof)]
/// SNARK proof of the protocol state at some point in time
pub struct ProtocolStateProof {
    pub statement: ProofStatement,
    pub prev_evals: PrevEvals,
    pub prev_x_hat: PrevXHat,
    pub proof: Proof,
}
impl_from_with_proxy!(
    ProtocolStateProof,
    ProtocolStateProofV1,
    ProtocolStateProofBase64Json
);
impl_from_with_proxy!(
    ProtocolStateProof,
    ProtocolStateProofV1,
    ProtocolStateProofJson
);

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofStatement)]
pub struct ProofStatement {
    pub proof_state: ProofState,
    pub pass_through: PairingBased,
}
impl_from_with_proxy!(ProofStatement, ProofStatementV1, ProofStatementJson);

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofState)]
pub struct ProofState {
    pub deferred_values: ProofStateDeferredValues,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluations,
    pub me_only: ProofStatePairingBased,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofStateDeferredValues)]
pub struct ProofStateDeferredValues {
    pub plonk: Plonk,
    pub combined_inner_product: ShiftedValue,
    pub b: ShiftedValue,
    pub xi: BulletproofPreChallenge,
    pub bulletproof_challenges: BulletproofChallengeTuple18,
    pub which_branch: Char,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::Plonk)]
pub struct Plonk {
    pub alpha: BulletproofPreChallenge,
    pub beta: ScalarChallengeVector2,
    pub gamma: ScalarChallengeVector2,
    pub zeta: BulletproofPreChallenge,
}

#[derive(Clone, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ShiftedValue)]
pub enum ShiftedValue {
    ShiftedValue(BigInt256),
}

impl Default for ShiftedValue {
    fn default() -> Self {
        Self::ShiftedValue(BigInt256::default())
    }
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::SpongeDigestBeforeEvaluations)]
pub struct SpongeDigestBeforeEvaluations(pub Hex64, pub Hex64, pub Hex64, pub Hex64, pub ());

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofStatePairingBased)]
pub struct ProofStatePairingBased {
    pub sg: FiniteECPoint,
    pub old_bulletproof_challenges: ProofStateBulletproofChallenges,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::PairingBased)]
pub struct PairingBased {
    pub app_state: (),
    pub sg: FiniteECPointVec,
    pub old_bulletproof_challenges: BulletproofChallenges,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::PrevEvals)]
pub struct PrevEvals(pub ProofEvaluations, pub ProofEvaluations);

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::PrevXHat)]
pub struct PrevXHat(pub FiniteECPoint);

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::Proof)]
pub struct Proof {
    pub messages: ProofMessages,
    pub openings: ProofOpenings,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofOpeningsEvals)]
pub struct ProofOpeningsEvals(pub ProofEvaluations, pub ProofEvaluations);

#[derive(Clone, Default, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_proof::ProofOpenings)]
pub struct ProofOpenings {
    pub proof: OpeningProof,
    pub evals: ProofOpeningsEvals,
}
