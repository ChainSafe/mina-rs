// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Functionality relating to the ledger or genesis ledgers

#![deny(warnings)]
#![deny(missing_docs)]

mod genesis_ledger;
pub use genesis_ledger::*;

#[cfg(not(target_arch = "wasm32"))]
mod rocksdb_genesis_ledger;
#[cfg(not(target_arch = "wasm32"))]
pub use rocksdb_genesis_ledger::RocksDbGenesisLedger;
