// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_serialization_types_macros::AutoFrom;

use crate::types::{ECPoint, ECPointVec, FiniteECPoint};

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::proof_messages::ProofMessages)]
pub struct ProofMessages {
    pub l_comm: ProofMessageWithoutDegreeBoundList,
    pub r_comm: ProofMessageWithoutDegreeBoundList,
    pub o_comm: ProofMessageWithoutDegreeBoundList,
    pub z_comm: ProofMessageWithoutDegreeBoundList,
    pub t_comm: ProofMessageWithDegreeBound,
}

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::proof_messages::ProofMessageWithoutDegreeBoundList)]
pub struct ProofMessageWithoutDegreeBoundList(pub Vec<FiniteECPoint>);

#[derive(Clone, Default, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::proof_messages::ProofMessageWithDegreeBound)]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ECPointVec,
    pub shifted: ECPoint,
}
