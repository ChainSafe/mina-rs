// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Proof messages used by the protocol state proof

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::field_and_curve_elements::{ECPointV1, ECPointVecV1, FiniteECPoint};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ProofMessages {
    pub l_comm: ProofMessageWithoutDegreeBoundListV1,
    pub r_comm: ProofMessageWithoutDegreeBoundListV1,
    pub o_comm: ProofMessageWithoutDegreeBoundListV1,
    pub z_comm: ProofMessageWithoutDegreeBoundListV1,
    pub t_comm: ProofMessageWithDegreeBoundV1,
}

pub type ProofMessagesV1 = Versioned<ProofMessages, 1>;

pub type ProofMessageWithoutDegreeBoundListV1 = Versioned<Versioned<Vec<FiniteECPoint>, 1>, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ProofMessageWithDegreeBound {
    pub unshifted: ECPointVecV1,
    pub shifted: ECPointV1,
}

pub type ProofMessageWithDegreeBoundV1 = Versioned<ProofMessageWithDegreeBound, 1>;