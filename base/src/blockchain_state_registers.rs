// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State Registers

use crate::numbers::*;
use mina_crypto::hash::*;
use proof_systems::*;

/// Mina blockchain state registers
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct BlockchainStateRegisters {
    /// TODO
    pub ledger: LedgerHash,
    /// TODO
    pub pending_coinbase_stack: (),
    /// TODO
    pub local_state: BlockchainStateRegistersLocalState,
}

impl ToChunkedROInput for BlockchainStateRegisters {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.ledger)
            .append_chunked(&self.local_state)
    }
}

/// Mina blockchain state register local state
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct BlockchainStateRegistersLocalState {
    /// TODO
    pub stack_frame: Field,
    /// TODO
    pub call_stack: Field,
    /// TODO
    pub transaction_commitment: Field,
    /// TODO
    pub full_transaction_commitment: Field,
    /// TODO
    pub token_id: TokenId,
    /// TODO
    pub excess: SignedAmount,
    /// TODO
    pub ledger: LedgerHash,
    /// TODO
    pub success: bool,
    /// TODO
    pub party_index: MinaIndex,
    /// TODO
    pub failure_status_tbl: Vec<()>,
}

impl ToChunkedROInput for BlockchainStateRegistersLocalState {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.stack_frame)
            .append_chunked(&self.call_stack)
            .append_chunked(&self.transaction_commitment)
            .append_chunked(&self.full_transaction_commitment)
            .append_chunked(&self.token_id)
            .append_chunked(&self.excess)
            .append_chunked(&self.ledger)
            .append_chunked(&self.party_index)
            .append_bool(self.success)
    }
}
