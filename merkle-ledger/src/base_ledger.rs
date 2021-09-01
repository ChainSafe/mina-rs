// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

use crate::location::Location;
use crate::path::Make as Path;
use crate::sincable;

pub enum CreateOutput {
    Added,
    Existed,
}

pub trait S: sincable::S {
    type Key;
    type TokenId;
    type TokenIdSet;
    type AccountId;
    type AccountIdSet;
    type Index;

    fn to_list(this: &Self::T) -> Vec<Self::Account>;
    // fn foldi_with_ignored_accounts;
    // fn iteri;
    // fn foldi;
    // fn fold_until;
    fn accounts(this: &Self::T) -> Self::AccountIdSet;
    fn token_owner(this: &Self::T, id: Self::TokenId) -> Option<Self::Key>;
    fn token_owners(this: &Self::T) -> Self::AccountIdSet;
    fn tokens(this: &Self::T, key: Self::Key) -> Self::TokenIdSet;
    fn next_available_token(this: &Self::T) -> Self::TokenId;
    fn set_next_available_token(this: &Self::T, id: Self::TokenId);
    fn location_of_account(this: &Self::T, id: Self::AccountId) -> Option<Location>;
    fn get_or_create_account(
        this: &Self::T,
        id: Self::AccountId,
        account: &Self::Account,
    ) -> anyhow::Result<(CreateOutput, Self::Account)>;
    // fn get_or_create_account_exn;
    fn close(this: &Self::T);
    // fn last_filled;
    fn get_uuid(this: &Self::T) -> Uuid;
    fn get_directory(this: &Self::T) -> Option<String>;
    fn get(this: &Self::T, location: &Location) -> Option<Self::Account>;
    fn set(this: &Self::T, location: &Location, account: Self::Account);
    fn set_batch(this: &Self::T, accounts: &[(Location, Self::Account)]);
    fn get_at_index_exn(this: &Self::T, index: usize) -> Self::Account;
    fn set_at_index_exn(this: &Self::T, index: usize, account: Self::Account);
    fn index_of_account_exn(this: &Self::T, id: Self::AccountId) -> usize;
    fn merkle_root(this: &Self::T) -> Self::RootHash;
    fn merkle_path(this: &Self::T, location: &Location) -> Path<Self::Hash, Self::H>;
    fn merkle_path_at_index_exn(this: &Self::T, index: usize) -> Path<Self::Hash, Self::H>;
    fn remove_accounts_exn(this: &Self::T, ids: &[Self::AccountId]);
    // fn detached_signal;
}
