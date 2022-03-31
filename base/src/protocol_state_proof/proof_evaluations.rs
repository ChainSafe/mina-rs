// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::FieldElementVec;


#[derive(Clone, Default, PartialEq, Debug)]
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
