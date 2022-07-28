// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module contains the Account data type
//! and associated types

pub mod permissions;
pub mod timing;
pub mod token_permissions;
pub mod token_symbol;
pub mod zkapp;

use crate::{types::*, *};

use mina_serialization_types_macros::AutoFrom;
pub use permissions::{AuthRequired, Permissions, PermissionsLegacy};
pub use timing::Timing;
pub use token_permissions::TokenPermissions;
pub use token_symbol::TokenSymbol;
pub use zkapp::{ZkApp, ZkAppUri};

use mina_crypto::hash::{ChainHash, StateHash};
use mina_hasher::ROInput;
use mina_serialization_types::account::*;
use proof_systems::{
    mina_hasher::{Hashable, Hasher},
    mina_signer::CompressedPubKey,
};

use self::zkapp::{ZkAppOptionHashableWrapper, ZkAppUriOptionHashableWrapper};

/// An account identified by its public key and token ID. Multiple accounts may
/// where the same public key if multiple tokens exist
///
/// Accounts can also be Snapps in which case snapp data is required and proofs must
/// be provided to perform certain actions
#[derive(Clone, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::account::Account)]
pub struct AccountLegacy {
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
    pub permissions: PermissionsLegacy,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub snapp: Option<()>,
}

impl BinProtSerializationType<'_> for AccountLegacy {
    type T = AccountV1;
}

impl mina_hasher::Hashable for AccountLegacy {
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
pub struct Account {
    /// Account public key
    pub public_key: CompressedPubKey,
    /// Account token ID
    pub token_id: TokenId,
    /// Balance of token held by account
    pub balance: Amount,
    /// Permission associated with the given token
    pub token_permissions: TokenPermissions,
    /// Token Symbol
    pub token_symbol: TokenSymbol,
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
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkapp: Option<ZkApp>,
    /// Level of permission required to do different account actions
    pub permissions: Permissions,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkapp_uri: Option<ZkAppUri>,
}

impl mina_hasher::Hashable for Account {
    type D = ();

    // Uncomment these fields once they have implemented Hashable trait
    // and add unit tests when it's complete
    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        let zkapp_hash = {
            let mut hasher = mina_hasher::create_kimchi(());
            hasher.hash(&ZkAppOptionHashableWrapper(&self.zkapp))
        };
        let zkapp_uri_hash = {
            let mut hasher = mina_hasher::create_kimchi(());
            hasher.hash(&ZkAppUriOptionHashableWrapper(&self.zkapp_uri))
        };
        roi.append_hashable(&CompressedPubKeyHashableWrapper(&self.public_key))
            .append_hashable(&self.token_id)
            .append_hashable(&self.balance)
            .append_hashable(&self.token_permissions)
            .append_hashable(&self.token_symbol)
            .append_hashable(&self.nonce)
            .append_hashable(&self.receipt_chain_hash)
            .append_hashable(&CompressedPubKeyOptionHashableWrapper(&self.delegate))
            .append_hashable(&self.voting_for)
            .append_hashable(&self.timing)
            .append_field(zkapp_hash)
            .append_hashable(&self.permissions)
            .append_field(zkapp_uri_hash);

        let mut roi = ROInput::new();
        // append_hashable_as_fields(&mut roi, &self.token_permissions);

        // append_hashable_as_fields(&mut roi, &self.balance);
        append_hashable_as_fields(&mut roi, &self.token_id);
        append_hashable_as_fields(&mut roi, &CompressedPubKeyHashableWrapper(&self.public_key));
        roi.append_hashable(&self.balance);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaAccount".into())
    }
}

impl BinProtSerializationType<'_> for Account {
    type T = AccountV0;
}

fn append_hashable_as_fields<T: Hashable>(roi: &mut ROInput, t: &T) {
    let mut tmp = ROInput::new();
    tmp.append_hashable(t);
    for f in tmp.to_fields().into_iter() {
        roi.append_field(f);
    }
}
