// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! types and functions related to Mina user_command

use crate::party::{Signed, Stable};
use crate::signed_command::*;
use crate::snapp_command::SnappCommand;
use crate::snapp_predicate::ProtocolState;
use crate::transaction_status::TransactionStatus;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
/// The user command with the transaction status
pub struct UserCommandWithStatus {
    /// User command
    pub data: UserCommand,
    /// Transaction status
    pub status: TransactionStatus,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/parties.ml#L7
pub struct Parties {
    /// User who payed the fee
    pub fee_payer: Signed,
    /// Trusted other parties
    pub other_parties: Stable,
    /// Snapp ProtocolState
    pub protocol_state: ProtocolState,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
#[non_exhaustive]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/user_command.ml#L6
/// User Command (Common Transactions)
pub enum UserCommand {
    /// Command for normal transaction
    SignedCommand(SignedCommand),
    /// Command for normal transaction
    Parties(Parties),
    /// Command for snapp transaction
    SnappCommand(SnappCommand),
}

impl Default for UserCommand {
    fn default() -> Self {
        Self::SignedCommand(SignedCommand::default())
    }
}
