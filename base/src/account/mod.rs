// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module contains the Account data type
//! and associated types

pub mod permissions;
pub mod timing;
pub mod token_permissions;

pub use permissions::{AuthRequired, Permissions};
pub use timing::Timing;
pub use token_permissions::TokenPermissions;

use crate::numbers::{AccountNonce, Amount, TokenId};
use mina_crypto::hash::{ChainHash, StateHash};
use mina_crypto::signature::PublicKey;
use serde::{Deserialize, Serialize};

use mina_serialization_types::v1::AccountV1;

/// An account identified by its public key and token ID. Multiple accounts may
/// where the same public key if multiple tokens exist
///
/// Accounts can also be Snapps in which case snapp data is required and proofs must
/// be provided to perform certain actions
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "AccountV1")]
#[serde(into = "AccountV1")]
pub struct Account {
    /// Account public key
    pub public_key: PublicKey,
    /// Account token ID
    pub token_id: TokenId,
    /// Permission associated with the given token
    pub token_permissions: TokenPermissions,
    /// Balance of token held by account
    pub balance: Amount,
    /// Nonce (incremented with each tx to prevent replay)
    pub nonce: AccountNonce,
    /// ?
    pub receipt_chain_hash: ChainHash,
    /// Delegate for staking purposes
    pub delegate: Option<PublicKey>,
    /// The state hash this account is voting for
    pub voting_for: StateHash,
    /// Any timing limitations places on this accounts balance
    /// Used for vesting
    pub timing: Timing,
    /// Level of permission required to do different account actions
    pub permissions: Permissions,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub snapp: Option<()>,
}
