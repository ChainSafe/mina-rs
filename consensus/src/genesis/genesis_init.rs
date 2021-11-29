// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_rs_base::types::ExternalTransition;

use super::*;

///
/// # Example
/// ```rust
/// use mina_rs_base::types::*;
/// use mina_consensus::genesis::{GenesisInit, DEVNET_CONFIG, MAINNET_CONFIG};
/// let genesis_mainnet = ExternalTransition::init_genesis(&MAINNET_CONFIG);
/// let genesis_devnet = ExternalTransition::init_genesis(&DEVNET_CONFIG);
/// ```
pub trait GenesisInit {
    fn init_genesis(config: &GenesisInitConfig) -> ExternalTransition;
}

impl GenesisInit for ExternalTransition {
    fn init_genesis(_config: &GenesisInitConfig) -> ExternalTransition {
        ExternalTransition::default()
    }
}
