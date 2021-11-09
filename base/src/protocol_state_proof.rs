// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod ark {

    use serde::{Deserialize, Serialize};

    use mina_curves::pasta::vesta::Affine as Vesta;
    use plonk_protocol_dlog::prover::ProverProof;

    #[derive(Clone, Serialize, Deserialize)]
    #[serde(from = "super::ProtocolStateProof")]
    #[serde(into = "super::ProtocolStateProof")]
    pub struct ProtocolStateProof(ProverProof<Vesta>);

    impl From<super::ProtocolStateProof> for ProtocolStateProof {
        fn from(_: super::ProtocolStateProof) -> Self {
            todo!()
        }
    }

    impl From<ProtocolStateProof> for super::ProtocolStateProof {
        fn from(_: ProtocolStateProof) -> Self {
            todo!()
        }
    }

    impl std::fmt::Debug for ProtocolStateProof {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            todo!()
        }
    }

    impl PartialEq for ProtocolStateProof {
        fn eq(&self, _: &Self) -> bool {
            todo!()
        }
    }
}

use ark_ec::AffineCurve;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use crate::numbers::{BigInt256, Char, Hex64};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 4)]
pub struct ProtocolStateProof {
    pub statement: ProofStatement,

    // polynomial evaluations
    pub prev_evals: PrevEvals,

    pub prev_x_hat: PrevXHat,

    // batched commitment opening proof
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
    pub sg: FiniteECPoint,
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
    pub sg: FiniteECPointVec,
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
pub struct PrevEvals((ProofEvaluations, ProofEvaluations));

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct PrevXHat(FiniteECPoint);

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
pub struct ProofMessageWithoutDegreeBoundList(Vec<FiniteECPoint>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ECPointVec,
    pub shifted: ECPoint,
}

impl<C> From<ProofMessageWithDegreeBound> for commitment_dlog::commitment::PolyComm<C>
where
    C: AffineCurve + std::convert::From<(BigInt256, BigInt256)>,
{
    fn from(t: ProofMessageWithDegreeBound) -> Self {
        todo!()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofOpenings {
    pub proof: OpeningProof,
    pub evals: (ProofEvaluations, ProofEvaluations),
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct OpeningProof {
    pub lr: FiniteECPointPairVec,
    pub z_1: FieldElement,
    pub z_2: FieldElement,
    pub delta: FiniteECPoint,
    pub sg: FiniteECPoint,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ProofEvaluations {
    pub l: FieldElementVec,
    pub r: FieldElementVec,
    pub o: FieldElementVec,
    pub z: FieldElementVec,
    pub t: FieldElementVec,
    pub f: FieldElementVec,
    pub sigma1: FieldElementVec,
    pub sigma2: FieldElementVec,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FieldElementVec(Vec<FieldElement>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointVec(Vec<FiniteECPoint>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointPairVec(Vec<(FiniteECPoint, FiniteECPoint)>);

pub type FiniteECPoint = (BigInt256, BigInt256); // point on elliptic curve, cannot encode the point at infinity
pub type FieldElement = BigInt256;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct ECPointVec(Vec<ECPoint>);

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub enum ECPoint {
    // elliptic curve point, can be the point at infinity
    Infinite,
    Finite(FiniteECPoint),
}

impl Default for ECPoint {
    fn default() -> Self {
        Self::Infinite
    }
}

use ark_ec::models::short_weierstrass_jacobian::GroupAffine;
use ark_ec::models::ModelParameters;

impl<P> From<ECPoint> for GroupAffine<P>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(p: ECPoint) -> Self {
        match p {
            ECPoint::Infinite => Self::new(Default::default(), Default::default(), true),
            ECPoint::Finite((x, y)) => Self::new(
                ark_ff::BigInteger256::from(x).into(),
                ark_ff::BigInteger256::from(y).into(),
                false,
            ),
        }
    }
}
