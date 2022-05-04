// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Transaction Snark Work
#![allow(missing_docs)]

use crate::types::*;
use mina_crypto::hash::*;
use proof_systems::mina_signer::CompressedPubKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionSnarkWork {
    pub fee: Amount,
    pub proofs: OneORTwo,
    pub prover: CompressedPubKey,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OneORTwo {
    // Versioned 1 byte
    One(Box<TransactionSnark>),
    Two(Box<TransactionSnark>, Box<TransactionSnark>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionSnark {
    pub statement: Statement,
    pub transaction_snark_proof: ProtocolStateProof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub source: StateHash,
    pub target: StateHash,
    pub supply_increase: Amount,
    pub pending_coinbase_stack_state: PendingCoinbaseStackState,
    pub fee_excess: FeeExcess,
    pub next_available_token_before: TokenId,
    pub next_available_token_after: TokenId,
    pub sok_digest: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingCoinbaseStackState {
    pub source: PendingCoinbase,
    pub target: PendingCoinbase,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingCoinbase {
    pub data_stack: StateHash,
    pub state_stack: StateStack,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StateStack {
    pub init: StateHash,
    pub curr: StateHash,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeeExcess {
    pub fee_token_l: TokenId,
    pub fee_excess_l: Signed,
    pub fee_token_r: TokenId,
    pub fee_excess_r: Signed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Signed {
    pub magnitude: Amount,
    pub sgn: SgnType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SgnType {
    Pos,
    Neg,
}
