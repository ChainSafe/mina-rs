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
///
pub struct UserCommandWithStatus {
    ///
    pub data: UserCommand,
    ///
    pub status: TransactionStatus,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/parties.ml#L7
pub struct Parties {
    ///
    pub fee_payer: Signed,
    ///
    pub other_parties: Stable,
    ///
    pub protocol_state: ProtocolState,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]
#[non_exhaustive]
/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/user_command.ml#L6
///
pub enum UserCommand {
    ///
    SignedCommand(SignedCommand),
    ///
    Parties(Parties),
    ///
    SnappCommand(SnappCommand),
}

impl Default for UserCommand {
    fn default() -> Self {
        Self::SignedCommand(SignedCommand::default())
    }
}
