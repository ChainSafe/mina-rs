// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module containing the components of a protocol state proof

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::{BigInt256, Char, Hex64};

pub(crate) mod proof_messages;
pub(crate) use proof_messages::ProofMessages;

pub(crate) mod proof_evaluations;
pub(crate) use proof_evaluations::ProofEvaluations;

pub(crate) mod opening_proof;
pub(crate) use opening_proof::OpeningProof;

pub(crate) mod bulletproof_challenges;
pub(crate) use bulletproof_challenges::{
    BulletproofChallengeTuple18,
    BulletproofChallenges, BulletproofPreChallenge, ProofStateBulletproofChallenges,
    ScalarChallengeVector2,
};

pub(crate) mod field_and_curve_elements;
pub(crate) use field_and_curve_elements::{
    ECPoint, ECPointVec, FieldElement, FieldElementVec, FiniteECPoint, FiniteECPointPairVec,
    FiniteECPointVec,
};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 4)]
/// SNARK proof of the protocol state at some point in time
pub struct ProtocolStateProof {
    pub(crate) statement: ProofStatement,
    pub(crate) prev_evals: PrevEvals,
    pub(crate) prev_x_hat: PrevXHat,
    pub(crate) proof: Proof,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub(crate) struct ProofStatement {
    pub(crate) proof_state: ProofState,
    pub(crate) pass_through: PairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct ProofState {
    pub(crate) deferred_values: ProofStateDeferredValues,
    pub(crate) sponge_digest_before_evaluations: SpongeDigestBeforeEvaluations,
    pub(crate) me_only: ProofStatePairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct ProofStateDeferredValues {
    pub(crate) plonk: Plonk,
    pub(crate) combined_inner_product: ShiftedValue,
    pub(crate) b: ShiftedValue,
    pub(crate) xi: BulletproofPreChallenge,
    pub(crate) bulletproof_challenges: BulletproofChallengeTuple18,
    pub(crate) which_branch: Char,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct Plonk {
    pub(crate) alpha: BulletproofPreChallenge,
    pub(crate) beta: ScalarChallengeVector2,
    pub(crate) gamma: ScalarChallengeVector2,
    pub(crate) zeta: BulletproofPreChallenge,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
pub(crate) enum ShiftedValue {
    ShiftedValue(BigInt256),
}

impl Default for ShiftedValue {
    fn default() -> Self {
        Self::ShiftedValue(BigInt256::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub(crate) struct SpongeDigestBeforeEvaluations((Hex64, Hex64, Hex64, Hex64, ()));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct ProofStatePairingBased {
    pub(crate) sg: FiniteECPoint,
    pub(crate) old_bulletproof_challenges: ProofStateBulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct PairingBased {
    pub(crate) app_state: (),
    pub(crate) sg: FiniteECPointVec,
    pub(crate) old_bulletproof_challenges: BulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct PrevEvals((ProofEvaluations, ProofEvaluations));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct PrevXHat(FiniteECPoint);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub(crate) struct Proof {
    pub(crate) messages: ProofMessages,
    pub(crate) openings: ProofOpenings,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub(crate) struct ProofOpenings {
    pub(crate) proof: OpeningProof,
    pub(crate) evals: (ProofEvaluations, ProofEvaluations),
}
