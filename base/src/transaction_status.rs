// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! types and functions related to Mina transaction_status
#![allow(missing_docs)]
use crate::numbers::{Amount, ExtendedU64_3};
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[non_exhaustive]
///
pub enum TransactionStatus {
    ///
    Applied(TransactionStatusApplied),
    ///
    Failed(TransactionStatusFailed),
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Applied(TransactionStatusApplied::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
///
pub struct TransactionStatusApplied((TransactionStatusAuxiliaryData, TransactionStatusBalanceData));
///
impl TransactionStatusApplied {
    ///
    pub fn auxiliary_data(&self) -> &TransactionStatusAuxiliaryData {
        &self.0 .0
    }
    ///
    pub fn balance_data(&self) -> &TransactionStatusBalanceData {
        &self.0 .1
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct TransactionStatusFailed((TransactionStatusFailure, TransactionStatusBalanceData));
///
impl TransactionStatusFailed {
    pub fn failure(&self) -> &TransactionStatusFailure {
        &self.0 .0
    }

    pub fn balance_data(&self) -> &TransactionStatusBalanceData {
        &self.0 .1
    }
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
///
pub struct TransactionStatusAuxiliaryData {
    pub fee_payer_account_creation_fee_paid: Option<Amount>,
    pub receiver_account_creation_fee_paid: Option<Amount>,
    pub created_token: Option<ExtendedU64_3>,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
///
pub struct TransactionStatusFailure {
    pub predicate: String,
    pub source_not_present: String,
    pub receiver_not_present: String,
    pub amount_insufficient_to_create_account: String,
    pub cannot_pay_creation_fee_in_token: String,
    pub source_insufficient_balance: String,
    pub source_minimum_balance_violation: String,
    pub receiver_already_exists: String,
    pub not_token_owner: String,
    pub mismatched_token_permissions: String,
    pub overflow: String,
    pub signed_command_on_snapp_account: String,
    pub snapp_account_not_present: String,
    pub update_not_permitted: String,
    pub incorrect_nonce: String,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
pub struct TransactionStatusBalanceData {
    pub fee_payer_balance: Option<ExtendedU64_3>,
    pub source_balance: Option<ExtendedU64_3>,
    pub receiver_balance: Option<ExtendedU64_3>,
}
