// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use super::*;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport, upgrade},
    identity,
    noise::{self, AuthenticKeypair, X25519Spec},
    pnet::PnetConfig,
    PeerId, Transport,
};
use libp2p_mplex::MplexConfig;
use std::{borrow::Borrow, time::Duration};

/// Type alias for libp2p transport
pub type P2PTransport = (PeerId, StreamMuxerBox);
/// Type alias for boxed libp2p transport
pub type BoxedP2PTransport = transport::Boxed<P2PTransport>;

/// Builds libp2p transport for mina with various configurations
#[derive(Clone)]
pub struct TransportBuilder {
    pnet_config: PnetConfig,
    mplex_config: MplexConfig,
    noise_keys: AuthenticKeypair<X25519Spec>,
    timeout: Duration,
    peer_id: PeerId,
}

impl TransportBuilder {
    /// Creates a new instance of [TransportBuilder] with random keypair and empty config
    pub fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        Self::new_with_key(keypair)
    }

    /// Creates a new instance of [TransportBuilder] with given keypair and empty config
    pub fn new_with_key(keypair: identity::Keypair) -> Self {
        Self::new_with_key_and_config(keypair, TransportConfig::default().borrow())
    }

    /// Creates a new instance of [TransportBuilder] with given keypair and config
    pub fn new_with_key_and_config(keypair: identity::Keypair, config: &TransportConfig) -> Self {
        let peer_id = PeerId::from(keypair.public());
        let shared_key = config.get_shared_key();
        let pnet_config = PnetConfig::new(shared_key);
        let mplex_config = config.get_mplex_config();
        let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&keypair)
            .expect("Signing libp2p-noise static DH keypair failed.");
        Self {
            pnet_config,
            mplex_config,
            noise_keys,
            timeout: Duration::from_secs(60),
            peer_id,
        }
    }

    /// Updates config for the [TransportBuilder] instance
    pub fn with_config(mut self, config: &TransportConfig) -> Self {
        let shared_key = config.get_shared_key();
        self.pnet_config = PnetConfig::new(shared_key);
        self.mplex_config = config.get_mplex_config();
        self
    }

    /// Uses mainnet config for the [TransportBuilder] instance
    pub fn with_mainnet_config(self) -> Self {
        self.with_config(&MAINNET_CONFIG)
    }

    /// Sets timeout duration for the [TransportBuilder] instance
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Builds libp2p transport
    pub fn build(self) -> Result<(BoxedP2PTransport, PeerId), std::io::Error> {
        let transport = {
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    use libp2p::wasm_ext;

                    // Note that DNS has been implictly supported in the extended javascript code,
                    // and TCP is not feasible in browsers
                    wasm_ext::ExtTransport::new(wasm_ext::ffi::websocket_transport())
                } else {
                    // Choose tokio over async-std here for 2 reasons:
                    //  1. Tokio has better performance as an coroutine scehduler.
                    //  2. TokioDnsConfig does not propagate async to build function's signature, while DnsConfig does.
                    // Cons:
                    //  1. Tokio builds more slowly.
                    //  2. Tokio's API is slightly more complicated.
                    //
                    use libp2p::{dns::TokioDnsConfig as DnsConfig, tcp::TokioTcpConfig as TcpConfig, websocket::WsConfig};

                    let tcp = TcpConfig::new().nodelay(true);
                    let dns_tcp = DnsConfig::system(tcp)?;
                    let ws_dns_tcp = WsConfig::new(dns_tcp.clone());
                    dns_tcp.or_transport(ws_dns_tcp)
                }
            }
        };

        Ok((
            transport
                .and_then(move |socket, _| self.pnet_config.handshake(socket))
                .upgrade(upgrade::Version::V1)
                .authenticate(noise::NoiseConfig::xx(self.noise_keys).into_authenticated())
                .multiplex(self.mplex_config)
                .timeout(self.timeout)
                .boxed(),
            self.peer_id,
        ))
    }
}

impl Default for TransportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
