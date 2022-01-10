// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use libp2p::pnet::PreSharedKey;
use libp2p_mplex::MplexConfig;
use multihash::{Blake2b256, StatefulHasher};

lazy_static::lazy_static! {
    pub static ref MAINNET_CONFIG : MinaTransportConfig<'static> = MinaTransportConfig
    {
        rendezvous_string: b"/coda/0.0.1/5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1",
        ..Default::default()
    };
}

#[derive(Clone, Debug)]
pub struct MinaTransportConfig<'a> {
    pub rendezvous_string: &'a [u8],
    pub mplex_protocol_name: &'static [u8],
}

impl<'a> MinaTransportConfig<'a> {
    pub fn get_shared_key(&self) -> PreSharedKey {
        let mut hasher = Blake2b256::default();
        hasher.update(self.rendezvous_string);
        let hash = hasher.finalize();
        let mut psk_fixed: [u8; 32] = Default::default();
        psk_fixed.copy_from_slice(hash.as_ref());
        PreSharedKey::new(psk_fixed)
    }

    pub fn get_mplex_config(&self) -> MplexConfig {
        let mut config = MplexConfig::new();
        config.set_protocol_name(self.mplex_protocol_name);
        config
    }
}

impl Default for MinaTransportConfig<'_> {
    fn default() -> Self {
        Self {
            rendezvous_string: b"",
            mplex_protocol_name: b"/coda/mplex/1.0.00",
        }
    }
}
