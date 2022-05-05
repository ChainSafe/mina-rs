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
use mina_serialization_types::v1::AccountV1;
use proof_systems::mina_signer::{BaseField, CompressedPubKey};
use proof_systems::o1_utils::FieldHelpers;

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
        // append compressed pubkey
        roi.append_field(self.public_key.x);
        roi.append_bool(self.public_key.is_odd);

        roi.append_hashable(&self.token_id)
            .append_hashable(&self.token_permissions)
            .append_hashable(&self.balance)
            .append_hashable(&self.nonce)
            .append_hashable(&self.receipt_chain_hash);
        match self.delegate {
            Some(c) => {
                roi.append_field(c.x);
                roi.append_bool(c.is_odd);
            }
            None => {
                let default_key = CompressedPubKey {
                    x: BaseField::from_bytes(&[0]).unwrap(), // FIXME: not sure if the default field should be this.
                    is_odd: false,
                };
                roi.append_field(default_key.x);
                roi.append_bool(default_key.is_odd);
            }
        }
        roi.append_hashable(&self.voting_for)
            .append_hashable(&self.timing)
            .append_hashable(&self.permissions);
        // .append_hashable(&self.snapp); //TODO enable when the correct type is available
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaAccount".into())
    }
}

mina_merkle::impl_poseidon_legacy_hasher_pool_provider!(Account);

#[cfg(test)]
mod tests {

    use crate::{
        mina_signer::PubKey,
        types::{Amount, BlockTime, Timing},
    };

    use super::timing::TimedData;
    #[test]
    fn account_roinput() {
        // {
        //     "pk": "B62qmqMrgPshhHKLJ7DqWn1KeizEgga5MuGmWb2bXajUnyivfeMW6JE",
        //     "balance": "372093",
        //     "delegate": "B62qrecVjpoZ4Re3a5arN6gXZ6orhmj1enUtA887XdG5mtZfdUbBUh4",
        //     "timing": {
        //       "initial_minimum_balance": "372093",
        //       "cliff_time": "86400",
        //       "cliff_amount": "372093",
        //       "vesting_period": "1",
        //       "vesting_increment": "0"
        //     }
        //   },

        let _pk = PubKey::from_address("B62qmqMrgPshhHKLJ7DqWn1KeizEgga5MuGmWb2bXajUnyivfeMW6JE")
            .expect("invalid source address");
        let _balance = 372093;
        let _delegate =
            PubKey::from_address("B62qrecVjpoZ4Re3a5arN6gXZ6orhmj1enUtA887XdG5mtZfdUbBUh4")
                .expect("invalid source address");
        let _timing = Timing::Timed(TimedData {
            initial_minimum_balance: Amount(372093),
            cliff_time: BlockTime(86400),
            cliff_amount: Amount(372093),
            vesting_period: BlockTime(1),
            vesting_increment: Amount(0),
        });
    }
}
