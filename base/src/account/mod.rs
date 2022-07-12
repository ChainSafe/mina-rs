// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module contains the Account data type
//! and associated types

pub mod permissions;
pub mod timing;
pub mod token_permissions;

use crate::{types::*, *};

use mina_serialization_types_macros::AutoFrom;
pub use permissions::{AuthRequired, Permissions};
pub use timing::Timing;
pub use token_permissions::TokenPermissions;

use mina_crypto::hash::{ChainHash, StateHash};
use mina_hasher::ROInput;
use mina_serialization_types::account::*;
use proof_systems::mina_signer::CompressedPubKey;

/// An account identified by its public key and token ID. Multiple accounts may
/// where the same public key if multiple tokens exist
///
/// Accounts can also be Snapps in which case snapp data is required and proofs must
/// be provided to perform certain actions
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::Account)]
pub struct Account {
    /// Account public key
    pub public_key: CompressedPubKey,
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
    pub delegate: Option<CompressedPubKey>,
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

impl BinProtSerializationType<'_> for Account {
    type T = AccountV1;
}

impl mina_hasher::Hashable for Account {
    type D = ();

    // Uncomment these fields once they have implemented Hashable trait
    // and add unit tests when it's complete
    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        roi
            // .append_hashable(self.public_key)
            .append_hashable(&self.token_id)
            // .append_hashable(self.token_permissions)
            .append_hashable(&self.balance)
            // .append_hashable(self.nonce)
            // .append_hashable(self.receipt_chain_hash)
            // .append_hashable(self.delegate)
            // .append_hashable(self.voting_for)
            // .append_hashable(self.timing)
            // .append_hashable(self.permissions)
            // .append_hashable(self.snapp)
            ;
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaAccount".into())
    }
}

/// TODO
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::AccountV0)]
pub struct AccountHardFork {
    /// Account public key
    pub public_key: CompressedPubKey,
    /// Account token ID
    pub token_id: TokenId,
    /// Permission associated with the given token
    pub token_permissions: TokenPermissions,
    /// Token Symbol
    pub token_symbol: [u8; 32],
    /// Balance of token held by account
    pub balance: Amount,
    /// Nonce (incremented with each tx to prevent replay)
    pub nonce: AccountNonce,
    /// ?
    pub receipt_chain_hash: ChainHash,
    /// Delegate for staking purposes
    pub delegate: Option<CompressedPubKey>,
    /// The state hash this account is voting for
    pub voting_for: StateHash,
    /// Any timing limitations places on this accounts balance
    /// Used for vesting
    pub timing: Timing,
    /// Level of permission required to do different account actions
    pub permissions: Permissions,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkapp: Option<()>,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkuri: Option<()>,
}

impl mina_hasher::Hashable for AccountHardFork {
    type D = ();

    // Uncomment these fields once they have implemented Hashable trait
    // and add unit tests when it's complete
    fn to_roinput(&self) -> ROInput {
        ROInput::new()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaAccount".into())
    }
}

impl BinProtSerializationType<'_> for AccountHardFork {
    type T = AccountV0;
}
