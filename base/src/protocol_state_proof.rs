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

use commitment_dlog::CommitmentField;
use ark_ff::PrimeField;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

use ark_ec::models::short_weierstrass_jacobian::GroupAffine;
use ark_ec::models::ModelParameters;

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

impl<P> From<ProtocolStateProof> for plonk_protocol_dlog::prover::ProverProof<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256> + PrimeField,
    <P as ModelParameters>::ScalarField: CommitmentField,
{
    fn from(t: ProtocolStateProof) -> Self {
        todo!()
        // Self {
        //     commitments: t.proof.messages.into(),
        //     proof: t.proof.openings.proof.into(),
        //     evals: t.proof.openings.evals.into(),
        //     // public:,
        //     // prev_challenges:,
        // }
    }
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

impl<P> From<ProofMessages> for plonk_protocol_dlog::prover::ProverCommitments<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(t: ProofMessages) -> Self {
        Self {
            l_comm: t.l_comm.into(),
            r_comm: t.r_comm.into(),
            o_comm: t.o_comm.into(),
            z_comm: t.z_comm.into(),
            t_comm: t.t_comm.into(),
        }
    }
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

impl<P> From<ProofMessageWithDegreeBound> for commitment_dlog::commitment::PolyComm<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(t: ProofMessageWithDegreeBound) -> Self {
        Self {
            unshifted: t.unshifted.0.into_iter().map(Into::into).collect(),
            shifted: Some(t.shifted.into()),
        }
    }
}

impl<P> From<ProofMessageWithoutDegreeBoundList>
    for commitment_dlog::commitment::PolyComm<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(t: ProofMessageWithoutDegreeBoundList) -> Self {
        Self {
            unshifted: t.0.into_iter().map(Into::into).collect(),
            shifted: None,
        }
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

impl<P> From<OpeningProof> for commitment_dlog::commitment::OpeningProof<GroupAffine<P>>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
    <P as ModelParameters>::ScalarField: From<ark_ff::BigInteger256>,
{
    fn from(t: OpeningProof) -> Self {
        Self {
            lr: t
                .lr
                .0
                .into_iter()
                .map(|(x, y)| (x.into(), y.into()))
                .collect(),
            delta: t.delta.into(),
            z1: ark_ff::BigInteger256::from(t.z_1).into(),
            z2: ark_ff::BigInteger256::from(t.z_2).into(),
            sg: t.sg.into(),
        }
    }
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

impl<Fs> From<ProofEvaluations> for plonk_circuits::scalars::ProofEvaluations<Vec<Fs>>
where
    Fs: From<ark_ff::BigInteger256>,
{
    fn from(t: ProofEvaluations) -> Self {
        Self {
            l: t.l.into(),
            r: t.r.into(),
            o: t.o.into(),
            z: t.z.into(),
            t: t.t.into(),
            f: t.f.into(),
            sigma1: t.sigma1.into(),
            sigma2: t.sigma2.into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FieldElementVec(Vec<FieldElement>);

impl<Fs> Into<Vec<Fs>> for FieldElementVec where Fs: From<ark_ff::BigInteger256> {
    fn into(self) -> Vec<Fs> {
        self.0.into_iter().map(|i| ark_ff::BigInteger256::from(i).into() ).collect()
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointVec(Vec<FiniteECPoint>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct FiniteECPointPairVec(Vec<(FiniteECPoint, FiniteECPoint)>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FiniteECPoint(FieldElement, FieldElement); // point on elliptic curve, cannot encode the point at infinity

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

impl<P> From<ECPoint> for GroupAffine<P>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(p: ECPoint) -> Self {
        match p {
            ECPoint::Infinite => Self::new(Default::default(), Default::default(), true),
            ECPoint::Finite(FiniteECPoint(x, y)) => Self::new(
                ark_ff::BigInteger256::from(x).into(),
                ark_ff::BigInteger256::from(y).into(),
                false,
            ),
        }
    }
}

impl<P> From<FiniteECPoint> for GroupAffine<P>
where
    P: ark_ec::SWModelParameters,
    <P as ModelParameters>::BaseField: From<ark_ff::BigInteger256>,
{
    fn from(FiniteECPoint(x, y): FiniteECPoint) -> Self {
        Self::new(
            ark_ff::BigInteger256::from(x).into(),
            ark_ff::BigInteger256::from(y).into(),
            false,
        )
    }
}
