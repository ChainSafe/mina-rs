// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_versions_derive::version;
use crate::numbers::currency;
use crate::token_id::TokenId;

#[derive(Clone, Serialize, Deserialize)]
enum TransactionStatusInner {
    Applied(AuxiliaryData, BalanceData),
    Failed(Failure, BalanceData),
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct TransactionStatus(TransactionStatusInner);

#[derive(Clone, Serialize, Deserialize)]
enum FailureInner {
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

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct Failure(FailureInner);


#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct AuxiliaryData {
    fee_payer_account_creation_fee_paid: Option<currency::Amount>,
    receiver_account_creation_fee_paid: Option<currency::Amount>,
    created_token: Option<TokenId>,
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct BalanceData {
    fee_payer_balance: Option<currency::Balance>,
    source_balance: Option<currency::Balance>,
    receiver_balance: Option<currency::Balance>,
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct WithStatus<T: Clone> {
    data: T,
    status: TransactionStatus
}

#[derive(Clone, Serialize, Deserialize)]
enum InternalCommandBalanceDataInner {
    Coinbase(CoinbaseBalanceData),
    FeeTransfer(FeeTransferBalanceData),
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct InternalCommandBalanceData(InternalCommandBalanceDataInner);

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct CoinbaseBalanceData {
    coinbase_receiver_balance: currency::Balance,
    fee_transfer_receiver_balance: currency::Balance,
}

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
struct FeeTransferBalanceData {
    receiver1_balance: currency::Balance,
    receiver2_balance: Option<currency::Balance>
}
