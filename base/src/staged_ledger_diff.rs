// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

// TODO: Get clarification on all the fields of this type before documenting
#![allow(missing_docs)]

use crate::types::TokenId;
use mina_crypto::signature::{PublicKey, Signature};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use derive_more::From;

use crate::numbers::{Amount, ExtendedU32};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff: StagedLedgerDiffTuple,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerDiffTuple(
    pub(crate) (StagedLedgerPreDiffTwo, Option<StagedLedgerPreDiffOne>),
);

impl StagedLedgerDiffTuple {
    pub fn diff_two(&self) -> &StagedLedgerPreDiffTwo {
        &self.0 .0
    }

    pub fn diff_one(&self) -> &Option<StagedLedgerPreDiffOne> {
        &self.0 .1
    }
}

// FIXME: No test coverage yet
pub type StagedLedgerPreDiffOne = ();

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerPreDiffTwo {
    pub completed_works: Vec<TransactionSnarkWork>,
    pub commands: Vec<UserCommandWithStatus>,
    pub coinbase: CoinBase,
    pub internal_command_balances: Vec<InternalCommandBalanceData>,
}

pub type TransactionSnarkWork = ();

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct UserCommandWithStatus {
    pub data: UserCommand,
    pub status: TransactionStatus,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum UserCommand {
    SignedCommand(SignedCommand),
    // FIXME: other variants are not covered by current test block
}

impl Default for UserCommand {
    fn default() -> Self {
        Self::SignedCommand(SignedCommand::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommand {
    pub payload: SignedCommandPayload,
    pub signer: PublicKey,
    pub signature: Signature,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommon,
    pub body: SignedCommandPayloadBody,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SignedCommandPayloadCommon {
    pub fee: Amount,
    pub fee_token: TokenId,
    pub fee_payer_pk: PublicKey,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemo,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum SignedCommandPayloadBody {
    PaymentPayload(PaymentPayload),
    // FIXME: other variants are not covered by current test block
}

impl Default for SignedCommandPayloadBody {
    fn default() -> Self {
        Self::PaymentPayload(PaymentPayload::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct PaymentPayload {
    pub source_pk: PublicKey,
    pub receiver_pk: PublicKey,
    pub token_id: TokenId,
    pub amount: Amount,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, From)]
pub struct SignedCommandMemo(pub Vec<u8>);

impl TryFrom<&str> for SignedCommandMemo {
    type Error = SignedCommandMemoError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        const DIGEST_LEN: usize = 32;
        const MAX_INPUT_STRING_LENGTH: usize = DIGEST_LEN;
        const MEMO_LEN: usize = DIGEST_LEN + 2;
        const TAG_INDEX: usize = 0;
        // const DIGEST_TAG: u8 = 0;
        const LEN_INDEX: usize = 1;
        const BYTES_TAG: u8 = 1;
        if s.len() > MAX_INPUT_STRING_LENGTH {
            return Err(SignedCommandMemoError::StringTooLong);
        }
        let mut v = vec![0; MEMO_LEN];
        v[TAG_INDEX] = BYTES_TAG;
        v[LEN_INDEX] = s.len() as u8;
        for (i, b) in s.as_bytes().iter().enumerate() {
            v[i + 2] = *b;
        }
        Ok(Self(v))
    }
}

impl TryFrom<String> for SignedCommandMemo {
    type Error = SignedCommandMemoError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

// TODO impl Into<String> for SignedCommandMemo

#[derive(Debug, Error)]
pub enum SignedCommandMemoError {
    #[error("Input string is too long")]
    StringTooLong,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
// FIXME: No test coverage yet
pub struct SnappCommand;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum TransactionStatus {
    Applied(TransactionStatusApplied),
    // FIXME: other variants are not covered by current test block
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Applied(TransactionStatusApplied::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusApplied(pub (TransactionStatusAuxiliaryData, TransactionStatusBalanceData));

impl TransactionStatusApplied {
    pub fn auxiliary_data(&self) -> &TransactionStatusAuxiliaryData {
        &self.0 .0
    }

    pub fn balance_data(&self) -> &TransactionStatusBalanceData {
        &self.0 .1
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<TokenId>,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<Amount>,
    pub source_balance: Option<Amount>,
    pub receiver_balance: Option<Amount>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: PublicKey,
    pub fee: Amount,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceData),
    FeeTransfer(FeeTransferBalanceData),
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: Amount,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<Amount>,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: Amount,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<Amount>,
}
