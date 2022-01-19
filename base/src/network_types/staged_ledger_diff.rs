// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

// TODO: Get clarification on all the fields of this type before documenting
#![allow(missing_docs)]

use mina_crypto::signature::{PublicKey2, PublicKey3, Signature};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use versioned::Versioned;

use crate::numbers::{Amount, ExtendedU32, ExtendedU64_2, ExtendedU64_3};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff: StagedLedgerDiffTupleV1,
}

pub struct StagedLedgerDiffV1(Versioned<StagedLedgerDiff, 1>);



#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerDiffTupleV1(Versioned<(StagedLedgerPreDiffTwoV1, Option<StagedLedgerPreDiffOneV1>), 1>);

// FIXME: No test coverage yet
pub type StagedLedgerPreDiffOneV1 = ();

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerPreDiffTwo {
    pub completed_works: Vec<()>, 
    pub commands: Vec<UserCommandWithStatus>,
    pub coinbase: CoinBase,
    pub internal_command_balances: Vec<InternalCommandBalanceData>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct StagedLedgerPreDiffTwoV1(Versioned<Versioned<StagedLedgerPreDiffTwo, 1>, 1>);


#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct UserCommandWithStatus {
    pub data: UserCommand,
    pub status: TransactionStatus,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct UserCommandWithStatusV1(Versioned<UserCommandWithStatus, 1>);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[wire_type(recurse = 2)]
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

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 2)]
pub struct SignedCommand {
    pub payload: SignedCommandPayload,
    pub signer: PublicKey3,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 2)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommon,
    pub body: SignedCommandPayloadBody,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 3)]
pub struct SignedCommandPayloadCommon {
    pub fee: Amount,
    pub fee_token: SignedCommandFeeToken,
    pub fee_payer_pk: PublicKey2,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemo,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[wire_type(recurse = 2)]
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

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 2)]
pub struct PaymentPayload {
    pub source_pk: PublicKey2,
    pub receiver_pk: PublicKey2,
    pub token_id: ExtendedU64_3,
    pub amount: Amount,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 3)]
pub struct SignedCommandFeeToken(pub u64);

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[wire_type(recurse = 2)]
// FIXME: No test coverage yet
pub struct SnappCommand;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusApplied((TransactionStatusAuxiliaryData, TransactionStatusBalanceData));

impl TransactionStatusApplied {
    pub fn auxiliary_data(&self) -> &TransactionStatusAuxiliaryData {
        &self.0 .0
    }

    pub fn balance_data(&self) -> &TransactionStatusBalanceData {
        &self.0 .1
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<ExtendedU64_3>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<ExtendedU64_3>,
    pub source_balance: Option<ExtendedU64_3>,
    pub receiver_balance: Option<ExtendedU64_3>,
}

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[wire_type(recurse = 2)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: PublicKey2,
    pub fee: ExtendedU64_2,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceData),
    FeeTransfer(FeeTransferBalanceData),
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<ExtendedU64_3>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<ExtendedU64_3>,
}
