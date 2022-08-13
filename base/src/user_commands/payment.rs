// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Payment commands are for transfering some token amounts between accounts

use crate::numbers::{Amount, TokenId};

use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_signer::CompressedPubKey;

/// The data specific to payload commands
#[derive(Clone, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::PaymentPayload)]
pub struct PaymentPayload {
    /// Account to transfer from
    pub source_pk: CompressedPubKey,
    /// Account to transfer to
    pub receiver_pk: CompressedPubKey,
    /// The token to transfer
    pub token_id: TokenId,
    /// The ammount of that token to transfer
    pub amount: Amount,
}
