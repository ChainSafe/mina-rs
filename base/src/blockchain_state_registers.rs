// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Blockchain State Registers

use crate::numbers::*;
use mina_crypto::hash::*;
use proof_systems::*;
use std::str::FromStr;

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
/// It will be retired after <https://github.com/MinaProtocol/mina/pull/11728> is merged
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl Default for BlockchainStateRegistersLocalState {
    fn default() -> Self {
        Self {
            stack_frame: Field::from_str_radix(
                "02F99BCFB0AA7F48C1888DA5A67196A2410FB084CD2DB1AF5216C5122AEBC054",
                16,
            )
            .unwrap(),
            call_stack: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )
            .unwrap(),
            transaction_commitment: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )
            .unwrap(),
            full_transaction_commitment: Field::from_str_radix(
                "0000000000000000000000000000000000000000000000000000000000000000",
                16,
            )
            .unwrap(),
            token_id: TokenId(1),
            excess: SignedAmount(0, true),
            ledger: LedgerHash::from_str("jw6bz2wud1N6itRUHZ5ypo3267stk4UgzkiuWtAMPRZo9g4Udyd")
                .unwrap(),
            success: true,
            party_index: MinaIndex(0),
            failure_status_tbl: Default::default(),
        }
    }
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
