// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

lazy_static::lazy_static! {
    pub static ref MAINNET_CONFIG: GenesisInitConfig = GenesisInitConfig::mainnet();
    pub static ref DEVNET_CONFIG: GenesisInitConfig = GenesisInitConfig::devnet();
}

pub struct GenesisInitConfig {}

impl GenesisInitConfig {
    pub(crate) fn mainnet() -> Self {
        Self {}
    }

    pub(crate) fn devnet() -> Self {
        Self {}
    }
}
