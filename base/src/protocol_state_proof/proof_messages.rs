// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::{ECPoint, ECPointVec, FiniteECPoint};

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ProofMessages {
    pub l_comm: ProofMessageWithoutDegreeBoundList,
    pub r_comm: ProofMessageWithoutDegreeBoundList,
    pub o_comm: ProofMessageWithoutDegreeBoundList,
    pub z_comm: ProofMessageWithoutDegreeBoundList,
    pub t_comm: ProofMessageWithDegreeBound,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ProofMessageWithoutDegreeBoundList(pub Vec<FiniteECPoint>);

#[derive(Clone, Default, PartialEq, Debug)]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ECPointVec,
    pub shifted: ECPoint,
}
