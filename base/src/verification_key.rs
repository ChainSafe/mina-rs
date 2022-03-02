// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! types and functions related to Mina verificiation keys

use serde::{Deserialize, Serialize};

use crate::protocol_state_proof::field_and_curve_elements::FiniteECPoint;

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
/// Public data required to verify a Mina snark
pub struct VerificationKey {
    commitments: VerificationKeyEvals,
    step_domains: Vec<Domains>,
    /// Associated data
    pub data: Data,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
struct Domains {
    h: Domain,
    x: Domain,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
enum Domain {
    Pow2RootsOfUnity(usize),
}

impl Default for Domain {
    fn default() -> Self {
        Self::Pow2RootsOfUnity(0)
    }
}

/// Data associated with a verification key
#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Data {
    /// Number of constaints
    pub constraints: usize,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
struct VerificationKeyEvals {
    sigma_comm_0: Vec<FiniteECPoint>,
    sigma_comm_1: Vec<FiniteECPoint>,
    sigma_comm_2: Vec<FiniteECPoint>,
    ql_comm: Vec<FiniteECPoint>,
    qr_comm: Vec<FiniteECPoint>,
    qo_comm: Vec<FiniteECPoint>,
    qm_comm: Vec<FiniteECPoint>,
    qc_comm: Vec<FiniteECPoint>,
    rcm_comm_0: Vec<FiniteECPoint>,
    rcm_comm_1: Vec<FiniteECPoint>,
    rcm_comm_2: Vec<FiniteECPoint>,
    psm_comm: Vec<FiniteECPoint>,
    add_comm: Vec<FiniteECPoint>,
    mul1_comm: Vec<FiniteECPoint>,
    mul2_comm: Vec<FiniteECPoint>,
    emul1_comm: Vec<FiniteECPoint>,
    emul2_comm: Vec<FiniteECPoint>,
    emul3_comm: Vec<FiniteECPoint>,
}
