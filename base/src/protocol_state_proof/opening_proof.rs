// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::{FieldElement, FiniteECPoint, FiniteECPointPairVec};


use ark_ec::models::ModelParameters;
use ark_ec::short_weierstrass_jacobian::GroupAffine;

#[derive(Clone, Default, PartialEq, Debug)]
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
            lr: t.lr.into(),
            delta: t.delta.into(),
            z1: ark_ff::BigInteger256::from(t.z_1).into(),
            z2: ark_ff::BigInteger256::from(t.z_2).into(),
            sg: t.sg.into(),
        }
    }
}
