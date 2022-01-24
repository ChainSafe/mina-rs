// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

#![allow(missing_docs)]

use mina_crypto::signature::{PublicKey2, PublicKey3, Signature};
use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::numbers::{Amount, ExtendedU32, ExtendedU64_2, ExtendedU64_3};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff: StagedLedgerDiffTupleV1,
}

pub type StagedLedgerDiffV1 = Versioned<StagedLedgerDiff, 1>;

pub type StagedLedgerDiffTupleV1 =
    Versioned<(StagedLedgerPreDiffTwoV1, Option<StagedLedgerPreDiffOneV1>), 1>;

// FIXME: No test coverage yet
pub type StagedLedgerPreDiffOneV1 = ();

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerPreDiffTwo {
    pub completed_works: Vec<()>,
    pub commands: Vec<UserCommandWithStatusV1>,
    pub coinbase: CoinBaseV1,
    pub internal_command_balances: Vec<InternalCommandBalanceDataV1>,
}

pub type StagedLedgerPreDiffTwoV1 = Versioned<Versioned<StagedLedgerPreDiffTwo, 1>, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct UserCommandWithStatus {
    pub data: UserCommandV1,
    pub status: TransactionStatusV1,
}

pub type UserCommandWithStatusV1 = Versioned<UserCommandWithStatus, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum UserCommand {
    SignedCommand(SignedCommandV1),
    // FIXME: other variants are not covered by current test block
}

impl Default for UserCommand {
    fn default() -> Self {
        Self::SignedCommand(SignedCommandV1::default())
    }
}

pub type UserCommandV1 = Versioned<Versioned<UserCommand, 1>, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommand {
    pub payload: SignedCommandPayloadV1,
    pub signer: PublicKey3,
    pub signature: Signature,
}

pub type SignedCommandV1 = Versioned<Versioned<SignedCommand, 1>, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommonV1,
    pub body: SignedCommandPayloadBodyV1,
}

pub type SignedCommandPayloadV1 = Versioned<Versioned<SignedCommandPayload, 1>, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayloadCommon {
    pub fee: Amount,
    pub fee_token: SignedCommandFeeTokenV1,
    pub fee_payer_pk: PublicKey2,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemoV1,
}

pub type SignedCommandPayloadCommonV1 =
    Versioned<Versioned<Versioned<SignedCommandPayloadCommon, 1>, 1>, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum SignedCommandPayloadBody {
    PaymentPayload(PaymentPayloadV1),
    // FIXME: other variants are not covered by current test block
}

pub type SignedCommandPayloadBodyV1 = Versioned<Versioned<SignedCommandPayloadBody, 1>, 1>;

impl Default for SignedCommandPayloadBody {
    fn default() -> Self {
        Self::PaymentPayload(PaymentPayloadV1::default())
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PaymentPayload {
    pub source_pk: PublicKey2,
    pub receiver_pk: PublicKey2,
    pub token_id: ExtendedU64_3,
    pub amount: Amount,
}

pub type PaymentPayloadV1 = Versioned<Versioned<PaymentPayload, 1>, 1>;

pub type SignedCommandFeeTokenV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

pub type SignedCommandMemoV1 = Versioned<Vec<u8>, 1>;

// FIXME: No test coverage yet
pub type SnappCommand = Versioned<Versioned<(), 1>, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum TransactionStatus {
    Applied(TransactionStatusAppliedV1),
    // FIXME: other variants are not covered by current test block
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Applied(TransactionStatusAppliedV1::default())
    }
}

pub type TransactionStatusV1 = Versioned<TransactionStatus, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusAppliedV1(
    pub  (
        TransactionStatusAuxiliaryDataV1,
        TransactionStatusBalanceDataV1,
    ),
);

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<ExtendedU64_3>,
}

pub type TransactionStatusAuxiliaryDataV1 = Versioned<TransactionStatusAuxiliaryData, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<ExtendedU64_3>,
    pub source_balance: Option<ExtendedU64_3>,
    pub receiver_balance: Option<ExtendedU64_3>,
}

pub type TransactionStatusBalanceDataV1 = Versioned<TransactionStatusBalanceData, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

pub type CoinBaseV1 = Versioned<CoinBase, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: PublicKey2,
    pub fee: ExtendedU64_2,
}

pub type CoinBaseFeeTransferV1 = Versioned<Versioned<CoinBaseFeeTransfer, 1>, 1>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceDataV1),
    FeeTransfer(FeeTransferBalanceDataV1),
}

pub type InternalCommandBalanceDataV1 = Versioned<InternalCommandBalanceData, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<ExtendedU64_3>,
}

pub type CoinBaseBalanceDataV1 = Versioned<CoinBaseBalanceData, 1>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<ExtendedU64_3>,
}

pub type FeeTransferBalanceDataV1 = Versioned<FeeTransferBalanceData, 1>;
