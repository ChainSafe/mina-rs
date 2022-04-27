// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This modules contains all genesis block related stuff
//!

mod config;
pub use config::*;
use mina_rs_base::types::*;
mod genesis_impl;

/// Trait for genesis block initialization logic
/// # Example
/// ```
/// use mina_rs_base::types::*;
/// use mina_consensus::genesis::*;
/// let genesis_mainnet = ExternalTransition::from_genesis_config(&MAINNET_CONFIG);
/// let genesis_devnet = ExternalTransition::from_genesis_config(&DEVNET_CONFIG);
/// ```
pub trait Genesis {
    /// Constructs a genesis block from config
    fn from_genesis_config(config: &GenesisInitConfig) -> ExternalTransition;
}
