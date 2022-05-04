// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! A Mina account record and supporting types
//! This isn't sent over the network but is serialized and stored in
//! kv stores, so to be compatible with those we need to support these types

use crate::v1::{
    AccountNonceV1, AmountV1, BlockTimeV1, HashV1, PublicKey2V1, PublicKeyV1, TokenIdV1,
};
use serde::{Deserialize, Serialize};
use versioned::Versioned;

/// An account as is serialized into the Mina ledger database stores
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// Account public key
    pub public_key: PublicKey2V1,
    /// Account token ID
    pub token_id: TokenIdV1,
    /// Permission associated with the given token
    pub token_permissions: TokenPermissionsV1,
    /// Balance of token held by account
    pub balance: AmountV1,
    /// Nonce (incremented with each tx to prevent replay)
    pub nonce: AccountNonceV1,
    /// ?
    pub receipt_chain_hash: HashV1,
    /// Delegate for staking purposes
    pub delegate: Option<PublicKeyV1>,
    /// The state hash this account is voting for
    pub voting_for: HashV1,
    /// Any timing limitations places on this accounts balance
    /// Used for vesting
    pub timing: TimingV1,
    /// Level of permission required to do different account actions
    pub permissions: PermissionsV1,
    /// TODO: This should contain a Snapp account data once we have something to test against
    pub snapp: Option<()>,
}

/// An account as is serialized into the Mina ledger database stores (v1)
pub type AccountV1 = Versioned<Versioned<Versioned<Account, 1>, 1>, 1>;

/// Need to learn exactly what this is..
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenPermissions {
    /// Account owns a token
    TokenOwned {
        /// ?
        disable_new_accounts: bool,
    },
    /// Account does not own a token
    NotOwned {
        /// ?
        account_disabled: bool,
    },
}

///
pub type TokenPermissionsV1 = Versioned<Versioned<TokenPermissions, 1>, 1>;

/// Permissions associated with the account
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Permissions {
    /// If the account can stake
    pub stake: bool,
    /// Permission required to edit state
    pub edit_state: AuthRequiredV1,
    /// Permission required to send a balance
    pub send: AuthRequiredV1,
    /// Permission required to receive balance
    pub receive: AuthRequiredV1,
    /// Permission required to set the delegate
    pub set_delegate: AuthRequiredV1,
    /// Permission required to cange permissions
    pub set_permissions: AuthRequiredV1,
    /// Permission require to set verification key
    pub set_verification_key: AuthRequiredV1,
}

/// Permissions associated with the account (v1)
pub type PermissionsV1 = Versioned<Versioned<Permissions, 1>, 1>;

/// The level of auth required to perform a particular action with an account
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AuthRequired {
    /// None required
    None,
    /// Either a proof or a signature
    Either,
    /// Proof must be provided
    Proof,
    /// Signature must be provided
    Signature,
    /// Both proof and signature must be provided
    Both,
    /// This action can never occur
    Impossible,
}

/// The level of auth required to perform a particular action with an account (v1)
pub type AuthRequiredV1 = Versioned<AuthRequired, 1>;

/// Timing information for an account with regard to when its balance is accessable
/// This is to allow vesting from an initial genesis allocation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Timing {
    /// Account does not have any timing limitations
    Untimed,
    /// Account does have timing limitations as specified
    Timed(TimedDataV1),
}

/// Payload for the timing variant Timed
/// Needs its own version byte
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimedData {
    /// Initial balance for the account
    pub initial_minimum_balance: AmountV1,
    /// Time when all balance is avaiable
    pub cliff_time: BlockTimeV1,
    /// Amount extra available when fully fested
    pub cliff_amount: AmountV1,
    /// Ammount released in each vesting period
    pub vesting_increment: AmountV1,
    /// Period in whcih allocation is released in chunks
    pub vesting_period: BlockTimeV1,
}

/// Payload for the timing variant Timed
/// Needs its own version byte (v1)
pub type TimedDataV1 = Versioned<TimedData, 1>;

/// Timing information for an account with regard to when its balance is accessable (v1)
pub type TimingV1 = Versioned<Versioned<Timing, 1>, 1>;