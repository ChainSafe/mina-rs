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

pub use self::zkapp::{ZkAppOptionHashableWrapper, ZkAppUriOptionHashableWrapper};
use mina_serialization_types_macros::AutoFrom;
pub use permissions::{AuthRequired, Permissions, PermissionsLegacy};
pub use timing::Timing;
pub use token_permissions::TokenPermissions;
pub use token_symbol::TokenSymbol;
pub use zkapp::{ZkApp, ZkAppUri};

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

// This implementation is not complete because we have switched to berkeley net
impl mina_hasher::Hashable for AccountLegacy {
    type D = ();

    // Uncomment these fields once they have implemented Hashable trait
    // and add unit tests when it's complete
    fn to_roinput(&self) -> ROInput {
        ROInput::new()
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
    /// Level of permission required to do different account actions
    pub permissions: Permissions,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkapp: Option<ZkApp>,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub zkapp_uri: Option<ZkAppUri>,
}

impl mina_hasher::Hashable for Account {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        self.roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaAccount".into())
    }
}

impl ToChunkedROInput for Account {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&ZkAppUriOptionHashableWrapper(&self.zkapp_uri))
            .append_chunked(&ZkAppOptionHashableWrapper(&self.zkapp))
            .append_chunked(&self.permissions)
            .append_chunked(&self.timing)
            .append_chunked(&self.voting_for)
            .append_chunked(&CompressedPubKeyOptionHashableWrapper(&self.delegate))
            .append_chunked(&self.receipt_chain_hash)
            .append_chunked(&self.nonce)
            .append_chunked(&self.balance)
            .append_chunked(&self.token_symbol)
            .append_chunked(&self.token_permissions)
            .append_chunked(&self.token_id)
            .append_chunked(&CompressedPubKeyHashableWrapper(&self.public_key))
    }
}

// TODO: No test coverage yet because there're new hash algo changes again
// that we are not able to follow anymore.
// Genesis ledger test data needs to be updated and tests have to be fixed first
impl FromGraphQLJson for Account {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let is_disabled = json["isDisabled"].as_bool().unwrap_or_default();
        let is_token_owner = json["isTokenOwner"].as_bool().unwrap_or_default();
        Ok(Self {
            public_key: CompressedPubKey::from_address(
                json["publicKey"].as_str().unwrap_or_default(),
            )?,
            // FIXME: wSHV2S4qX9jFsLjQo8r1BsMLH2ZRKsZx6EJd1sbozGPieEC4Jf
            token_id: TokenId(json["token"].as_str().unwrap_or_default().parse()?),
            balance: Amount(
                json["balance"]["total"]
                    .as_str()
                    .unwrap_or_default()
                    .parse()?,
            ),
            token_permissions: if is_token_owner {
                TokenPermissions::TokenOwned {
                    disable_new_accounts: is_disabled,
                }
            } else {
                TokenPermissions::NotOwned {
                    account_disabled: is_disabled,
                }
            },
            // FIXME: figure out what this is
            token_symbol: Default::default(),
            nonce: AccountNonce(json["nonce"].as_str().unwrap_or_default().parse()?),
            receipt_chain_hash: ChainHash::from_str(
                json["receiptChainHash"].as_str().unwrap_or_default(),
            )?,
            delegate: match json["delegate"].as_str() {
                Some(s) => Some(CompressedPubKey::from_address(s)?),
                _ => None,
            },
            voting_for: StateHash::from_str(json["votingFor"].as_str().unwrap_or_default())?,
            timing: Timing::from_graphql_json(&json["timing"])?,
            permissions: Permissions::from_graphql_json(&json["permissions"])?,
            // FIXME: struct to be defined
            zkapp: None,
            // FIXME: struct to be defined
            zkapp_uri: None,
        })
    }
}

impl BinProtSerializationType<'_> for Account {
    type T = AccountV0;
}
