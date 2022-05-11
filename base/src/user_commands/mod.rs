// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! User commands are how external users can mutate the state of the Mina ledger
//! This module contains the command data structures and helpers to create and
//! serialize new commands to broadcast to the network

pub mod memo;
pub mod payment;
pub mod signed_command;

pub use memo::SignedCommandMemo;
pub use payment::PaymentPayload;
pub use signed_command::{
    SignedCommand, SignedCommandPayload, SignedCommandPayloadBody, SignedCommandPayloadCommon,
};

use mina_serialization_types::json::UserCommandJson;
use mina_serialization_types_macros::AutoFrom;
use versioned::*;

/// The top level user command type
/// This is the output of the command builders
#[derive(Clone, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::staged_ledger_diff::UserCommand)]
pub enum UserCommand {
    /// A command signed by a private key
    SignedCommand(SignedCommand),
    // FIXME: other variants are not covered by current test block
}

impl_from_with_proxy!(
    UserCommand,
    mina_serialization_types::staged_ledger_diff::UserCommand,
    UserCommandJson
);
