// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Transaction Snark Work
#![allow(missing_docs)]

use crate::types::*;
use mina_crypto::hash::*;
use mina_serialization_types::json::*;
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_signer::CompressedPubKey;
use versioned::*;

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::TransactionSnarkWork)]
pub struct TransactionSnarkWork {
    pub fee: Amount,
    pub proofs: OneORTwo,
    pub prover: CompressedPubKey,
}

impl_from_with_proxy!(
    TransactionSnarkWork,
    mina_serialization_types::snark_work::TransactionSnarkWork,
    TransactionSnarkWorkJson
);

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::OneORTwo)]
pub enum OneORTwo {
    // Versioned 1 byte
    One(Box<TransactionSnark>),
    Two(Box<TransactionSnark>, Box<TransactionSnark>),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::TransactionSnark)]
pub struct TransactionSnark {
    pub statement: Statement,
    pub transaction_snark_proof: ProtocolStateProof,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::Statement)]
pub struct Statement {
    pub source: StateHash,
    pub target: StateHash,
    pub supply_increase: Amount,
    pub pending_coinbase_stack_state: PendingCoinbaseStackState,
    pub fee_excess: FeeExcess,
    pub next_available_token_before: TokenId,
    pub next_available_token_after: TokenId,
    pub sok_digest: ByteVec,
}

impl_from_with_proxy!(
    Statement,
    mina_serialization_types::snark_work::Statement,
    StatementJson
);

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::PendingCoinbaseStackState)]
pub struct PendingCoinbaseStackState {
    pub source: PendingCoinbase,
    pub target: PendingCoinbase,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::PendingCoinbase)]
pub struct PendingCoinbase {
    pub data_stack: StateHash,
    pub state_stack: StateStack,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::StateStack)]
pub struct StateStack {
    pub init: StateHash,
    pub curr: StateHash,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::FeeExcess)]
pub struct FeeExcess {
    pub fee_token_l: TokenId,
    pub fee_excess_l: Signed,
    pub fee_token_r: TokenId,
    pub fee_excess_r: Signed,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::Signed)]
pub struct Signed {
    pub magnitude: Amount,
    pub sgn: SgnType,
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::snark_work::SgnType)]
pub enum SgnType {
    Pos,
    Neg,
}
