// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

// TODO: Get clarification on all the fields of this type before documenting
#![allow(missing_docs)]

use crate::numbers::Amount;
use crate::snark_work::TransactionSnarkWork;
use crate::types::{SignedCommandPayload, TokenId};
use crate::user_commands::UserCommand;
use crate::verifiable::Verifiable;

use proof_systems::mina_signer::{CompressedPubKey, Signer};

#[derive(Clone, PartialEq, Debug, Default)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff_two: StagedLedgerPreDiff,
    pub diff_one: Option<StagedLedgerPreDiff>,
}

impl<CTX> Verifiable<CTX> for StagedLedgerDiff
where
    CTX: Signer<SignedCommandPayload>,
{
    // StagedLedgerDiff is considered valid if:
    // - All PreDiffs are valid
    fn verify(&self, ctx: &mut CTX) -> bool {
        if let Some(diff_one) = &self.diff_one {
            diff_one.verify(ctx) && self.diff_two.verify(ctx)
        } else {
            self.diff_two.verify(ctx)
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct StagedLedgerPreDiff {
    pub completed_works: Vec<TransactionSnarkWork>,
    pub commands: Vec<UserCommandWithStatus>,
    pub coinbase: CoinBase,
    pub internal_command_balances: Vec<InternalCommandBalanceData>,
}

impl<CTX> Verifiable<CTX> for StagedLedgerPreDiff
where
    CTX: Signer<SignedCommandPayload>,
{
    // PreDiff is considered valid if:
    // - all commands are valid
    fn verify(&self, ctx: &mut CTX) -> bool {
        self.commands
            .iter()
            .all(|cmd_with_status| cmd_with_status.data.verify(ctx))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UserCommandWithStatus {
    pub data: UserCommand,
    pub status: TransactionStatus,
}

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum TransactionStatus {
    Applied(TransactionStatusApplied),
    // FIXME: other variants are not covered by current test block
}

#[derive(Clone, PartialEq, Debug)]
pub struct TransactionStatusApplied(
    pub (TransactionStatusAuxiliaryData, TransactionStatusBalanceData),
);

impl TransactionStatusApplied {
    pub fn auxiliary_data(&self) -> &TransactionStatusAuxiliaryData {
        &self.0 .0
    }

    pub fn balance_data(&self) -> &TransactionStatusBalanceData {
        &self.0 .1
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<TokenId>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<Amount>,
    pub source_balance: Option<Amount>,
    pub receiver_balance: Option<Amount>,
}

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum CoinBase {
    Zero,
    // FIXME: other variants are not covered by current test block
    One(Option<CoinBaseFeeTransfer>),
    Two,
}

impl Default for CoinBase {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Clone, PartialEq, Debug)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: CompressedPubKey,
    pub fee: Amount,
}

#[derive(Clone, PartialEq, Debug)]
#[non_exhaustive]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceData),
    FeeTransfer(FeeTransferBalanceData),
}

#[derive(Clone, PartialEq, Debug)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: Amount,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<Amount>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: Amount,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<Amount>,
}
