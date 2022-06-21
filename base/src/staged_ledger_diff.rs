// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

// TODO: Get clarification on all the fields of this type before documenting
#![allow(missing_docs)]

use crate::numbers::Amount;
use crate::snark_work::TransactionSnarkWork;
use crate::types::TokenId;
use crate::user_commands::{SignedCommandPayload, UserCommand};
use crate::verifiable::Verifiable;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::mina_signer::{CompressedPubKey, Signer};
use smart_default::SmartDefault;
use versioned::*;

#[derive(Clone, PartialEq, Debug, Default, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::StagedLedgerDiff)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff: StagedLedgerDiffTuple,
}

impl_from_with_proxy!(StagedLedgerDiff, StagedLedgerDiffV1, StagedLedgerDiffJson);

impl<CTX> Verifiable<CTX> for StagedLedgerDiff
where
    CTX: Signer<SignedCommandPayload>,
{
    // StagedLedgerDiff is considered valid if:
    // - All PreDiffs are valid
    fn verify(&self, ctx: &mut CTX) -> bool {
        if let Some(diff_one) = &self.diff.diff_one() {
            diff_one.verify(ctx) && self.diff.diff_two().verify(ctx)
        } else {
            self.diff.diff_two().verify(ctx)
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::StagedLedgerDiffTuple)]
pub struct StagedLedgerDiffTuple(
    pub(crate) StagedLedgerPreDiff,
    pub(crate) Option<StagedLedgerPreDiff>,
);

impl StagedLedgerDiffTuple {
    pub fn diff_two(&self) -> &StagedLedgerPreDiff {
        &self.0
    }

    pub fn diff_one(&self) -> &Option<StagedLedgerPreDiff> {
        &self.1
    }
}

#[derive(Clone, PartialEq, Debug, Default, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::StagedLedgerPreDiff)]
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

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::UserCommandWithStatus)]
pub struct UserCommandWithStatus {
    pub data: UserCommand,
    pub status: TransactionStatus,
}

impl_from_with_proxy!(
    UserCommandWithStatus,
    mina_serialization_types::staged_ledger_diff::UserCommandWithStatus,
    UserCommandWithStatusJson
);

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::TransactionStatus)]
pub enum TransactionStatus {
    Applied(TransactionStatusAuxiliaryData, TransactionStatusBalanceData),
    Failed(
        Vec<TransactionStatusFailedType>,
        TransactionStatusBalanceData,
    ),
}

impl_from_with_proxy!(
    TransactionStatus,
    mina_serialization_types::staged_ledger_diff::TransactionStatus,
    TransactionStatusJson
);

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::TransactionStatusAuxiliaryData)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<TokenId>,
}

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::TransactionStatusBalanceData)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<Amount>,
    pub source_balance: Option<Amount>,
    pub receiver_balance: Option<Amount>,
}

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::TransactionStatusFailedType)]
pub enum TransactionStatusFailedType {
    Predicate,
    SourceNotPresent,
    ReceiverNotPresent,
    AmountInsufficientToCreateAccount,
    CannotPayCreationFeeInToken,
    SourceInsufficientBalance,
    SourceMinimumBalanceViolation,
    ReceiverAlreadyExists,
    NotTokenOwner,
    MismatchedTokenPermissions,
    Overflow,
    SignedCommandOnSnappAccount,
    SnappAccountNotPresent,
    UpdateNotPermitted,
    IncorrectNonce,
}

impl_from_with_proxy!(
    TransactionStatusFailedType,
    mina_serialization_types::staged_ledger_diff::TransactionStatusFailedType,
    TransactionStatusFailedTypeJson
);

#[derive(Clone, PartialEq, Debug, SmartDefault, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::CoinBase)]
pub enum CoinBase {
    #[default]
    Zero,
    One(Option<CoinBaseFeeTransfer>),
    Two(Option<CoinBaseFeeTransfer>, Option<CoinBaseFeeTransfer>),
}

impl_from_with_proxy!(
    CoinBase,
    mina_serialization_types::staged_ledger_diff::CoinBase,
    CoinBaseJson
);

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::CoinBaseFeeTransfer)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: CompressedPubKey,
    pub fee: Amount,
}

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::InternalCommandBalanceData)]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceData),
    FeeTransfer(FeeTransferBalanceData),
}

impl_from_with_proxy!(
    InternalCommandBalanceData,
    mina_serialization_types::staged_ledger_diff::InternalCommandBalanceData,
    InternalCommandBalanceDataJson
);

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::CoinBaseBalanceData)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: Amount,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<Amount>,
}

#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::FeeTransferBalanceData)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: Amount,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<Amount>,
}
