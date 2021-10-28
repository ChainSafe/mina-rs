// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::{BigInt256, Char, Hex64};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 4)]
pub struct ProtocolStateProof {
    pub statement: ProofStatement,
    pub prev_evals: PrevEvals,
    pub prev_x_hat: PrevXHat,
    pub proof: Proof,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProofStatement {
    pub proof_state: ProofState,
    pub pass_through: PairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofState {
    pub deferred_values: ProofStateDeferredValues,
    pub sponge_digest_before_evaluations: SpongeDigestBeforeEvaluations,
    pub me_only: ProofStatePairingBased,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofStateDeferredValues {
    pub plonk: Plonk,
    pub combined_inner_product: ShiftedValue,
    pub b: ShiftedValue,
    pub xi: BulletproofPreChallenge,
    pub bulletproof_challenges: BulletproofChallengeTuple18,
    pub which_branch: Char,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct Plonk {
    pub alpha: BulletproofPreChallenge,
    pub beta: ScalarChallengeVector2,
    pub gamma: ScalarChallengeVector2,
    pub zeta: BulletproofPreChallenge,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
pub enum ShiftedValue {
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
pub struct SpongeDigestBeforeEvaluations((Hex64, Hex64, Hex64, Hex64, ()));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofStatePairingBased {
    pub sg: BackendCommonHashTuple,
    pub old_bulletproof_challenges: ProofStateBulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofStateBulletproofChallenges(
    (BulletproofChallengeTuple17, BulletproofChallengeTuple17, ()),
);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PairingBased {
    pub app_state: (),
    pub sg: BackendCommonHashTupleList,
    pub old_bulletproof_challenges: BulletproofChallenges,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BulletproofChallenges(Vec<BulletproofChallengeTuple18>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 3)]
pub struct BulletproofChallengeTuple17(
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    (),
);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct BulletproofChallengeTuple18(
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    BulletproofChallenge,
    (),
);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BulletproofChallenge {
    pub prechallenge: BulletproofPreChallenge,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
pub enum BulletproofPreChallenge {
    ScalarChallenge(ScalarChallengeVector2),
}

#[derive(Clone, Serialize, Default, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ScalarChallengeVector2((Hex64, Hex64, ()));

impl Default for BulletproofPreChallenge {
    fn default() -> Self {
        Self::ScalarChallenge(ScalarChallengeVector2::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PrevEvals((ProofOpeningsEval, ProofOpeningsEval));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PrevXHat(BackendCommonHashTuple);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct Proof {
    pub messages: ProofMessages,
    pub openings: ProofOpenings,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessages {
    pub l_comm: ProofMessageWithoutDegreeBoundList,
    pub r_comm: ProofMessageWithoutDegreeBoundList,
    pub o_comm: ProofMessageWithoutDegreeBoundList,
    pub z_comm: ProofMessageWithoutDegreeBoundList,
    pub t_comm: ProofMessageWithDegreeBound,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
pub struct ProofMessageWithoutDegreeBoundList(Vec<ProofMessageWithoutDegreeBound>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofMessageWithoutDegreeBound((BigInt256, BigInt256));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ProofMessageWithDegreeBoundFiniteOrInfiniteList,
    pub shifted: ProofMessageWithDegreeBoundFiniteOrInfinite,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessageWithDegreeBoundFiniteOrInfiniteList(
    Vec<ProofMessageWithDegreeBoundFiniteOrInfinite>,
);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
pub enum ProofMessageWithDegreeBoundFiniteOrInfinite {
    Infinite,
    Finite(ProofMessageWithoutDegreeBound),
}

impl Default for ProofMessageWithDegreeBoundFiniteOrInfinite {
    fn default() -> Self {
        Self::Infinite
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpenings {
    pub proof: ProofOpeningsProof,
    pub evals: ProofOpeningsEvalTuple,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpeningsProof {
    pub lr: BackendCommonHashTupleTupleList,
    pub z_1: BigInt256,
    pub z_2: BigInt256,
    pub delta: BackendCommonHashTuple,
    pub sg: BackendCommonHashTuple,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpeningsEval {
    pub l: BackendCommonHashList,
    pub r: BackendCommonHashList,
    pub o: BackendCommonHashList,
    pub z: BackendCommonHashList,
    pub t: BackendCommonHashList,
    pub f: BackendCommonHashList,
    pub sigma1: BackendCommonHashList,
    pub sigma2: BackendCommonHashList,
}

pub type ProofOpeningsEvalTuple = (ProofOpeningsEval, ProofOpeningsEval);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BackendCommonHashList(Vec<BigInt256>);

pub type BackendCommonHashTuple = (BigInt256, BigInt256);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BackendCommonHashTupleList(Vec<BackendCommonHashTuple>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct BackendCommonHashTupleTupleList(Vec<(BackendCommonHashTuple, BackendCommonHashTuple)>);
