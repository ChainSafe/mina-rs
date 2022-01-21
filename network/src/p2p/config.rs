// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use libp2p::pnet::PreSharedKey;
use libp2p_mplex::MplexConfig;
use multihash::{Blake2b256, StatefulHasher};

lazy_static::lazy_static! {
    /// Mainnet config for mina p2p network
    pub static ref MAINNET_CONFIG : TransportConfig<'static> = TransportConfig
    {
        rendezvous_string: b"/coda/0.0.1/5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1",
        ..Default::default()
    };
}

/// Configuration type for [super::MinaTransportBuilder]
#[derive(Clone, Debug)]
pub struct TransportConfig<'a> {
    /// Rendezvous string for configuring private network
    pub rendezvous_string: &'a [u8],
    /// Protocol name for configuring libp2p multiplexer
    pub mplex_protocol_name: &'static [u8],
}

impl<'a> TransportConfig<'a> {
    /// Gets [PreSharedKey] from the [MinaTransportConfig] instance
    pub fn get_shared_key(&self) -> PreSharedKey {
        let mut hasher = Blake2b256::default();
        hasher.update(self.rendezvous_string);
        let hash = hasher.finalize();
        let mut psk_fixed: [u8; 32] = Default::default();
        psk_fixed.copy_from_slice(hash.as_ref());
        PreSharedKey::new(psk_fixed)
    }

    /// Gets [MplexConfig] from the [MinaTransportConfig] instance
    pub fn get_mplex_config(&self) -> MplexConfig {
        let mut config = MplexConfig::new();
        config.set_protocol_name(self.mplex_protocol_name);
        config
    }
}

impl Default for TransportConfig<'_> {
    fn default() -> Self {
        Self {
            rendezvous_string: b"",
            mplex_protocol_name: b"/coda/mplex/1.0.0",
        }
    }
}
