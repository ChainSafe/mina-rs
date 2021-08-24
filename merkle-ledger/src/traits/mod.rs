// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod account;
pub mod account_id;
pub mod balance;
pub mod hash;
pub mod key;
pub mod key_value_database;
pub mod token_id;

pub mod depth {
    pub trait Depth {
        fn depth(&self) -> usize;
    }
}

pub trait StorageLocations {
    fn key_value_db_dir() -> String;
}
