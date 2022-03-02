// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::{ECPoint, ECPointVec, FiniteECPoint};
use serde::{Deserialize, Serialize};

use ark_ec::models::short_weierstrass_jacobian::GroupAffine;
use ark_ec::models::ModelParameters;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofMessageWithoutDegreeBoundList(pub Vec<FiniteECPoint>);

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
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
            unshifted: t.unshifted.into(),
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
