// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module containing the components of a protocol state proof

// Much of this crate will be replaced by arkworks and 01-proof-systems types
// so full documentation will not be included
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use crate::numbers::{BigInt256, Char, Hex64};

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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
/// SNARK proof of the protocol state at some point in time
pub struct ProtocolStateProof {
    pub statement: ProofStatement,
    pub prev_evals: PrevEvals,
    pub prev_x_hat: PrevXHat,
    pub proof: Proof,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofStatement {
    pub proof_state: ProofState,
    pub pass_through: PairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofState {
    pub deferred_values: ProofStateDeferredValues,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluations,
    pub me_only: ProofStatePairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofStateDeferredValues {
    pub plonk: Plonk,
    pub combined_inner_product: ShiftedValue,
    pub b: ShiftedValue,
    pub xi: BulletproofPreChallenge,
    pub bulletproof_challenges: BulletproofChallengeTuple18,
    pub which_branch: Char,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Plonk {
    pub alpha: BulletproofPreChallenge,
    pub beta: ScalarChallengeVector2,
    pub gamma: ScalarChallengeVector2,
    pub zeta: BulletproofPreChallenge,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum ShiftedValue {
    ShiftedValue(BigInt256),
}

impl Default for ShiftedValue {
    fn default() -> Self {
        Self::ShiftedValue(BigInt256::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SpongeDigestBeforeEvaluations(pub (Hex64, Hex64, Hex64, Hex64, ()));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofStatePairingBased {
    pub sg: FiniteECPoint,
    pub old_bulletproof_challenges: ProofStateBulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PairingBased {
    pub app_state: (),
    pub sg: FiniteECPointVec,
    pub old_bulletproof_challenges: BulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PrevEvals(pub (ProofEvaluations, ProofEvaluations));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PrevXHat(pub FiniteECPoint);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Proof {
    pub messages: ProofMessages,
    pub openings: ProofOpenings,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofOpenings {
    pub proof: OpeningProof,
    pub evals: (ProofEvaluations, ProofEvaluations),
}
