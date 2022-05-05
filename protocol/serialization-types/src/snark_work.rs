// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Transaction Snark Work
#![allow(missing_docs)]

use crate::v1::{AmountV1, ByteVecV1, HashV1, ProtocolStateProofV1, PublicKeyV1, TokenIdV1};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionSnarkWork {
    // Versioned 1 byte
    pub fee: AmountV1,
    pub proofs: OneORTwoV1,
    pub prover: PublicKeyV1,
}

pub type TransactionSnarkWorkV1 = Versioned<TransactionSnarkWork, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "Polyvar")]
pub enum OneORTwo {
    // Versioned 1 byte
    #[serde(rename = "One")]
    One(Box<LedgerProofV1>),
    #[serde(rename = "Two")]
    Two(Box<LedgerProofV1>, Box<LedgerProofV1>),
}

pub type OneORTwoV1 = Versioned<OneORTwo, 1>;

pub type LedgerProofV1 = Versioned<TransactionSnarkV1, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionSnark {
    // Versioned 1 byte
    pub statement: StatementV1,
    pub transaction_snark_proof: ProtocolStateProofV1,
}

pub type TransactionSnarkV1 = Versioned<TransactionSnark, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Statement {
    // Versioned 2 byte
    pub source: HashV1,
    pub target: HashV1,
    pub supply_increase: AmountV1,
    pub pending_coinbase_stack_state: PendingCoinbaseStackStateV1,
    pub fee_excess: FeeExcessV1,
    pub next_available_token_before: TokenIdV1,
    pub next_available_token_after: TokenIdV1,
    pub sok_digest: ByteVecV1,
}

pub type StatementV1 = Versioned<Versioned<Statement, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PendingCoinbaseStackState {
    // Versioned 2 byte
    pub source: PendingCoinbaseV1,
    pub target: PendingCoinbaseV1,
}

pub type PendingCoinbaseStackStateV1 = Versioned<Versioned<PendingCoinbaseStackState, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PendingCoinbase {
    // Versioned 2 byte
    pub data_stack: HashV1,
    pub state_stack: StateStackV1,
}

pub type PendingCoinbaseV1 = Versioned<Versioned<PendingCoinbase, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateStack {
    // Versioned 2 byte
    pub init: HashV1,
    pub curr: HashV1,
}

pub type StateStackV1 = Versioned<Versioned<StateStack, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeeExcess {
    // Versioned 2 byte
    pub fee_token_l: TokenIdV1,
    pub fee_excess_l: SignedV1,
    pub fee_token_r: TokenIdV1,
    pub fee_excess_r: SignedV1,
}

pub type FeeExcessV1 = Versioned<Versioned<FeeExcess, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Signed {
    // Versioned 1 byte
    pub magnitude: AmountV1,
    pub sgn: SgnTypeV1,
}

pub type SignedV1 = Versioned<Signed, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SgnType {
    // Versioned 1 byte
    Pos,
    Neg,
}

pub type SgnTypeV1 = Versioned<SgnType, 1>;
