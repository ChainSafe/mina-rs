// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! In this context a diff refers to a difference between two states of the blockchain.
//! In this case it is between the current state and the proposed next state.

#![allow(missing_docs)] // Don't actually know what many of the types fields are for yet

use crate::{common::*, json::*, signatures::*, v1::*, version_bytes, *};
use mina_serialization_types_macros::AutoFrom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smart_default::SmartDefault;
use versioned::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Top level wrapper type for a StagedLedgerDiff
pub struct StagedLedgerDiff {
    pub diff: StagedLedgerDiffTupleV1,
}

/// Top level wrapper type for a StagedLedgerDiff (v1)
pub type StagedLedgerDiffV1 = Versioned<StagedLedgerDiff, 1>;

/// Top level wrapper type for a StagedLedgerDiff (json)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(StagedLedgerDiff)]
pub struct StagedLedgerDiffJson {
    pub diff: StagedLedgerDiffTupleJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StagedLedgerDiffTuple(pub StagedLedgerPreDiffV1, pub Option<StagedLedgerPreDiffV1>);

pub type StagedLedgerDiffTupleV1 = Versioned<StagedLedgerDiffTuple, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(StagedLedgerDiffTuple)]
pub struct StagedLedgerDiffTupleJson(
    pub StagedLedgerPreDiffJson,
    pub Option<StagedLedgerPreDiffJson>,
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StagedLedgerPreDiff {
    pub completed_works: Vec<TransactionSnarkWorkV1>,
    pub commands: Vec<UserCommandWithStatusV1>,
    pub coinbase: CoinBaseV1,
    pub internal_command_balances: Vec<InternalCommandBalanceDataV1>,
}

pub type StagedLedgerPreDiffV1 = Versioned<Versioned<StagedLedgerPreDiff, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(StagedLedgerPreDiff)]
pub struct StagedLedgerPreDiffJson {
    pub completed_works: Vec<TransactionSnarkWorkJson>,
    pub commands: Vec<UserCommandWithStatusJson>,
    pub coinbase: CoinBaseJson,
    pub internal_command_balances: Vec<InternalCommandBalanceDataJson>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserCommandWithStatus {
    pub data: UserCommandV1,
    pub status: TransactionStatusV1,
}

pub type UserCommandWithStatusV1 = Versioned<UserCommandWithStatus, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(UserCommandWithStatus)]
pub struct UserCommandWithStatusJson {
    pub data: UserCommandJson,
    pub status: TransactionStatusJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserCommand {
    SignedCommand(SignedCommandV1),
    // FIXME: other variants are not covered by current test block
}

pub type UserCommandV1 = Versioned<Versioned<UserCommand, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum UserCommandJsonProxy {
    #[serde(rename = "Signed_command")]
    SignedCommand(SignedCommandJson),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(UserCommand)]
#[auto_from(UserCommandJsonProxy)]
pub enum UserCommandJson {
    SignedCommand(SignedCommandJson),
}

impl_mina_enum_json_serde!(UserCommandJson, UserCommandJsonProxy);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommand {
    pub payload: SignedCommandPayloadV1,
    pub signer: PublicKey2V1,
    pub signature: SignatureV1,
}

pub type SignedCommandV1 = Versioned<Versioned<SignedCommand, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(SignedCommand)]
pub struct SignedCommandJson {
    pub payload: SignedCommandPayloadJson,
    pub signer: PublicKeyJson,
    pub signature: SignatureJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommandPayload {
    pub common: SignedCommandPayloadCommonV1,
    pub body: SignedCommandPayloadBodyV1,
}

pub type SignedCommandPayloadV1 = Versioned<Versioned<SignedCommandPayload, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(SignedCommandPayload)]
pub struct SignedCommandPayloadJson {
    pub common: SignedCommandPayloadCommonJson,
    pub body: SignedCommandPayloadBodyJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommandPayloadCommon {
    pub fee: AmountV1,
    pub fee_token: SignedCommandFeeTokenV1,
    pub fee_payer_pk: PublicKeyV1,
    pub nonce: ExtendedU32,
    pub valid_until: ExtendedU32,
    pub memo: SignedCommandMemoV1,
}

pub type SignedCommandPayloadCommonV1 =
    Versioned<Versioned<Versioned<SignedCommandPayloadCommon, 1>, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(SignedCommandPayloadCommon)]
pub struct SignedCommandPayloadCommonJson {
    pub fee: DecimalJson,
    pub fee_token: U64Json,
    pub fee_payer_pk: PublicKeyJson,
    pub nonce: U32Json,
    pub valid_until: U32Json,
    pub memo: SignedCommandMemoJson,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SignedCommandPayloadBody {
    PaymentPayload(PaymentPayloadV1),
    StakeDelegation(StakeDelegationV1),
    // FIXME: other variants are not covered by current test block
}

pub type SignedCommandPayloadBodyV1 = Versioned<Versioned<SignedCommandPayloadBody, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum SignedCommandPayloadBodyJsonProxy {
    #[serde(rename = "Payment")]
    PaymentPayload(PaymentPayloadJson),
    #[serde(rename = "Stake_delegation")]
    StakeDelegation(StakeDelegationJson),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(SignedCommandPayloadBody)]
#[auto_from(SignedCommandPayloadBodyJsonProxy)]
pub enum SignedCommandPayloadBodyJson {
    PaymentPayload(PaymentPayloadJson),
    StakeDelegation(StakeDelegationJson),
}

impl_mina_enum_json_serde_with_option!(
    SignedCommandPayloadBodyJson,
    SignedCommandPayloadBodyJsonProxy,
    false
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentPayload {
    pub source_pk: PublicKeyV1,
    pub receiver_pk: PublicKeyV1,
    pub token_id: ExtendedU64_3,
    pub amount: AmountV1,
}

pub type PaymentPayloadV1 = Versioned<Versioned<PaymentPayload, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(PaymentPayload)]
pub struct PaymentPayloadJson {
    pub source_pk: PublicKeyJson,
    pub receiver_pk: PublicKeyJson,
    pub token_id: U64Json,
    pub amount: U64Json,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StakeDelegation {
    SetDelegate {
        delegator: PublicKeyV1,
        new_delegate: PublicKeyV1,
    },
}

pub type StakeDelegationV1 = Versioned<StakeDelegation, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum StakeDelegationJsonProxy {
    #[serde(rename = "Set_delegate")]
    SetDelegate {
        delegator: PublicKeyJson,
        new_delegate: PublicKeyJson,
    },
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(StakeDelegation)]
#[auto_from(StakeDelegationJsonProxy)]
pub enum StakeDelegationJson {
    SetDelegate {
        delegator: PublicKeyJson,
        new_delegate: PublicKeyJson,
    },
}

impl_mina_enum_json_serde!(StakeDelegationJson, StakeDelegationJsonProxy);

pub type SignedCommandFeeTokenV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SignedCommandMemo(pub Vec<u8>);

pub type SignedCommandMemoV1 = Versioned<SignedCommandMemo, 1>;

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(SignedCommandMemo)]
pub struct SignedCommandMemoJson(pub Vec<u8>);

impl Serialize for SignedCommandMemoJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = bs58::encode(self.0.as_slice())
            .with_check_version(version_bytes::USER_COMMAND_MEMO)
            .into_string();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for SignedCommandMemoJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = bs58::decode(s)
            .with_check(Some(version_bytes::USER_COMMAND_MEMO))
            .into_vec()
            .map_err(<D::Error as serde::de::Error>::custom)?;
        // Skip base58 check byte
        Ok(Self(decoded.into_iter().skip(1).collect()))
    }
}

// FIXME: No test coverage yet
pub type SnappCommand = Versioned<Versioned<(), 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Applied(
        TransactionStatusAuxiliaryDataV1,
        TransactionStatusBalanceDataV1,
    ),
    Failed(
        Vec<TransactionStatusFailedTypeV1>,
        TransactionStatusBalanceDataV1,
    ),
}

pub type TransactionStatusV1 = Versioned<TransactionStatus, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum TransactionStatusJsonProxy {
    Applied(
        TransactionStatusAuxiliaryDataJson,
        TransactionStatusBalanceDataJson,
    ),
    Failed(
        Vec<TransactionStatusFailedTypeJson>,
        TransactionStatusBalanceDataJson,
    ),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(TransactionStatus)]
#[auto_from(TransactionStatusJsonProxy)]
pub enum TransactionStatusJson {
    Applied(
        TransactionStatusAuxiliaryDataJson,
        TransactionStatusBalanceDataJson,
    ),
    Failed(
        Vec<TransactionStatusFailedTypeJson>,
        TransactionStatusBalanceDataJson,
    ),
}

impl_mina_enum_json_serde!(TransactionStatusJson, TransactionStatusJsonProxy);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<AmountV1>,
    pub receiver_account_creation_fee_paid: Option<AmountV1>,
    pub created_token: Option<ExtendedU64_3>,
}

pub type TransactionStatusAuxiliaryDataV1 = Versioned<TransactionStatusAuxiliaryData, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(TransactionStatusAuxiliaryData)]
pub struct TransactionStatusAuxiliaryDataJson {
    pub fee_payer_account_creation_fee_paid: Option<U64Json>,
    pub receiver_account_creation_fee_paid: Option<U64Json>,
    pub created_token: Option<U64Json>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatusFailedType {
    #[serde(rename = "Predicate")]
    Predicate,
    #[serde(rename = "Source_not_present")]
    SourceNotPresent,
    #[serde(rename = "Receiver_not_present")]
    ReceiverNotPresent,
    #[serde(rename = "Amount_insufficient_to_create_account")]
    AmountInsufficientToCreateAccount,
    #[serde(rename = "Cannot_pay_creation_fee_in_token")]
    CannotPayCreationFeeInToken,
    #[serde(rename = "Source_insufficient_balance")]
    SourceInsufficientBalance,
    #[serde(rename = "Source_minimum_balance_violation")]
    SourceMinimumBalanceViolation,
    #[serde(rename = "Receiver_already_exists")]
    ReceiverAlreadyExists,
    #[serde(rename = "Not_token_owner")]
    NotTokenOwner,
    #[serde(rename = "Mismatched_token_permissions")]
    MismatchedTokenPermissions,
    #[serde(rename = "Overflow")]
    Overflow,
    #[serde(rename = "Signed_command_on_snapp_account")]
    SignedCommandOnSnappAccount,
    #[serde(rename = "Snapp_account_not_present")]
    SnappAccountNotPresent,
    #[serde(rename = "Update_not_permitted")]
    UpdateNotPermitted,
    #[serde(rename = "Incorrect_nonce")]
    IncorrectNonce,
}
pub type TransactionStatusFailedTypeV1 = Versioned<TransactionStatusFailedType, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum TransactionStatusFailedTypeJsonProxy {
    #[serde(rename = "Predicate")]
    Predicate,
    #[serde(rename = "Source_not_present")]
    SourceNotPresent,
    #[serde(rename = "Receiver_not_present")]
    ReceiverNotPresent,
    #[serde(rename = "Amount_insufficient_to_create_account")]
    AmountInsufficientToCreateAccount,
    #[serde(rename = "Cannot_pay_creation_fee_in_token")]
    CannotPayCreationFeeInToken,
    #[serde(rename = "Source_insufficient_balance")]
    SourceInsufficientBalance,
    #[serde(rename = "Source_minimum_balance_violation")]
    SourceMinimumBalanceViolation,
    #[serde(rename = "Receiver_already_exists")]
    ReceiverAlreadyExists,
    #[serde(rename = "Not_token_owner")]
    NotTokenOwner,
    #[serde(rename = "Mismatched_token_permissions")]
    MismatchedTokenPermissions,
    #[serde(rename = "Overflow")]
    Overflow,
    #[serde(rename = "Signed_command_on_snapp_account")]
    SignedCommandOnSnappAccount,
    #[serde(rename = "Snapp_account_not_present")]
    SnappAccountNotPresent,
    #[serde(rename = "Update_not_permitted")]
    UpdateNotPermitted,
    #[serde(rename = "Incorrect_nonce")]
    IncorrectNonce,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(TransactionStatusFailedType)]
#[auto_from(TransactionStatusFailedTypeJsonProxy)]
pub enum TransactionStatusFailedTypeJson {
    #[serde(rename = "Predicate")]
    Predicate,
    #[serde(rename = "Source_not_present")]
    SourceNotPresent,
    #[serde(rename = "Receiver_not_present")]
    ReceiverNotPresent,
    #[serde(rename = "Amount_insufficient_to_create_account")]
    AmountInsufficientToCreateAccount,
    #[serde(rename = "Cannot_pay_creation_fee_in_token")]
    CannotPayCreationFeeInToken,
    #[serde(rename = "Source_insufficient_balance")]
    SourceInsufficientBalance,
    #[serde(rename = "Source_minimum_balance_violation")]
    SourceMinimumBalanceViolation,
    #[serde(rename = "Receiver_already_exists")]
    ReceiverAlreadyExists,
    #[serde(rename = "Not_token_owner")]
    NotTokenOwner,
    #[serde(rename = "Mismatched_token_permissions")]
    MismatchedTokenPermissions,
    #[serde(rename = "Overflow")]
    Overflow,
    #[serde(rename = "Signed_command_on_snapp_account")]
    SignedCommandOnSnappAccount,
    #[serde(rename = "Snapp_account_not_present")]
    SnappAccountNotPresent,
    #[serde(rename = "Update_not_permitted")]
    UpdateNotPermitted,
    #[serde(rename = "Incorrect_nonce")]
    IncorrectNonce,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<ExtendedU64_3>,
    pub source_balance: Option<ExtendedU64_3>,
    pub receiver_balance: Option<ExtendedU64_3>,
}

pub type TransactionStatusBalanceDataV1 = Versioned<TransactionStatusBalanceData, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(TransactionStatusBalanceData)]
pub struct TransactionStatusBalanceDataJson {
    pub fee_payer_balance: Option<U64Json>,
    pub source_balance: Option<U64Json>,
    pub receiver_balance: Option<U64Json>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SmartDefault)]
pub enum CoinBase {
    #[default]
    Zero,
    // FIXME: other variants are not covered by current test block
    One(Option<CoinBaseFeeTransferV1>),
    Two,
}

pub type CoinBaseV1 = Versioned<CoinBase, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, SmartDefault)]
enum CoinBaseJsonProxy {
    #[default]
    Zero,
    One(Option<CoinBaseFeeTransferJson>),
    Two,
}

#[derive(Clone, Debug, PartialEq, SmartDefault, AutoFrom)]
#[auto_from(CoinBase)]
#[auto_from(CoinBaseJsonProxy)]
pub enum CoinBaseJson {
    #[default]
    Zero,
    One(Option<CoinBaseFeeTransferJson>),
    Two,
}

impl_mina_enum_json_serde!(CoinBaseJson, CoinBaseJsonProxy);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
// FIXME: No test coverage yet
pub struct CoinBaseFeeTransfer {
    pub receiver_pk: PublicKeyV1,
    pub fee: ExtendedU64_2,
}

pub type CoinBaseFeeTransferV1 = Versioned<Versioned<CoinBaseFeeTransfer, 1>, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(CoinBaseFeeTransfer)]
pub struct CoinBaseFeeTransferJson {
    pub receiver_pk: PublicKeyJson,
    pub fee: U64Json,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum InternalCommandBalanceData {
    CoinBase(CoinBaseBalanceDataV1),
    FeeTransfer(FeeTransferBalanceDataV1),
}

pub type InternalCommandBalanceDataV1 = Versioned<InternalCommandBalanceData, 1>;

#[derive(Clone, Debug, Serialize, Deserialize)]
enum InternalCommandBalanceDataJsonProxy {
    #[serde(rename = "Coinbase")]
    CoinBase(CoinBaseBalanceDataJson),
    #[serde(rename = "Fee_transfer")]
    FeeTransfer(FeeTransferBalanceDataJson),
}

#[derive(Clone, Debug, PartialEq, AutoFrom)]
#[auto_from(InternalCommandBalanceData)]
#[auto_from(InternalCommandBalanceDataJsonProxy)]
pub enum InternalCommandBalanceDataJson {
    CoinBase(CoinBaseBalanceDataJson),
    FeeTransfer(FeeTransferBalanceDataJson),
}

impl_mina_enum_json_serde!(
    InternalCommandBalanceDataJson,
    InternalCommandBalanceDataJsonProxy
);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CoinBaseBalanceData {
    pub coinbase_receiver_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub fee_transfer_receiver_balance: Option<ExtendedU64_3>,
}

pub type CoinBaseBalanceDataV1 = Versioned<CoinBaseBalanceData, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(CoinBaseBalanceData)]
pub struct CoinBaseBalanceDataJson {
    pub coinbase_receiver_balance: U64Json,
    pub fee_transfer_receiver_balance: Option<U64Json>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeeTransferBalanceData {
    pub receiver1_balance: ExtendedU64_3,
    // FIXME: No test coverage yet
    pub receiver2_balance: Option<ExtendedU64_3>,
}

pub type FeeTransferBalanceDataV1 = Versioned<FeeTransferBalanceData, 1>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, AutoFrom)]
#[auto_from(FeeTransferBalanceData)]
pub struct FeeTransferBalanceDataJson {
    pub receiver1_balance: U64Json,
    pub receiver2_balance: Option<U64Json>,
}
