// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Helpers for building a user command

use proof_systems::mina_signer::CompressedPubKey;

use crate::numbers::{AccountNonce, Amount, GlobalSlotNumber, TokenId};
use crate::user_commands::{
    PaymentPayload, SignedCommandPayload, SignedCommandPayloadBody, SignedCommandPayloadCommon,
};

use super::SignedCommandMemo;

/// A builder for UserCommands to transfer funds
pub struct SignedTransferCommandBuilder {
    to: CompressedPubKey,
    from: CompressedPubKey,
    amount: Amount,
    transfer_token: TokenId,
    fee_token: TokenId,
    fee: Amount,
    nonce: AccountNonce,
    memo: SignedCommandMemo,
    fee_payer_pk: CompressedPubKey,
    valid_until: GlobalSlotNumber,
}

impl SignedTransferCommandBuilder {
    /// All required fields must be defined initially
    pub fn new<T: Into<Amount>, TT: Into<Amount>, TTT: Into<AccountNonce>>(
        from: CompressedPubKey,
        to: CompressedPubKey,
        amount: T,
        fee: TT,
        nonce: TTT,
    ) -> Self {
        Self {
            to,
            from,
            amount: amount.into(),
            transfer_token: TokenId(1),
            fee_token: TokenId(1),
            fee: fee.into(),
            nonce: nonce.into(),
            fee_payer_pk: from,
            memo: SignedCommandMemo::default(),
            valid_until: GlobalSlotNumber::MAX,
        }
    }

    /// Set token to transfer
    pub fn transfer_token<T: Into<TokenId>>(self, transfer_token: T) -> Self {
        Self {
            transfer_token: transfer_token.into(),
            ..self
        }
    }

    /// Set the fee token to pay the block producer
    pub fn fee_token<T: Into<TokenId>>(self, fee_token: T) -> Self {
        Self {
            fee_token: fee_token.into(),
            ..self
        }
    }

    /// Set the fee payer to something other than the sender
    pub fn fee_payer(self, fee_payer_pk: CompressedPubKey) -> Self {
        Self {
            fee_payer_pk,
            ..self
        }
    }

    /// Set a non-empty memo for the command
    pub fn memo(self, memo: SignedCommandMemo) -> Self {
        Self { memo, ..self }
    }

    /// Set the global slot which this command is valid until
    pub fn valid_until<T: Into<GlobalSlotNumber>>(self, valid_until: T) -> Self {
        Self {
            valid_until: valid_until.into(),
            ..self
        }
    }

    /// Sign the transaction and produce a UserCommand with the signature fields filled
    pub fn build(self) -> SignedCommandPayload {
        SignedCommandPayload {
            common: SignedCommandPayloadCommon {
                fee: self.fee,
                fee_token: self.fee_token,
                memo: self.memo,
                fee_payer_pk: self.fee_payer_pk,
                nonce: self.nonce,
                valid_until: self.valid_until,
            },
            body: SignedCommandPayloadBody::PaymentPayload(PaymentPayload {
                amount: self.amount,
                receiver_pk: self.to,
                source_pk: self.from,
                token_id: self.transfer_token,
            }),
        }
    }
}
