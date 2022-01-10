// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use libp2p::{futures::StreamExt, ping, swarm::SwarmEvent, tcp::TcpConfig, Multiaddr, Swarm};
    use mina_network::p2p::{MinaTransportBuilder, MinaTransportConfig, MAINNET_CONFIG};

    const DEST: &str =
        "/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb";

    #[ignore = "flacky"]
    #[test]
    pub fn mainnet() {
        let config = MAINNET_CONFIG.clone();
        test_with_config(&config, 10, true);
    }

    #[test]
    pub fn dummy_mplex() {
        let mut config = MAINNET_CONFIG.clone();
        config.mplex_protocol_name = b"dummy";
        test_with_config(&config, 5, false);
    }

    #[test]
    pub fn dummy_rendezvous() {
        let mut config: MinaTransportConfig = Default::default();
        config.rendezvous_string = b"dummy";
        test_with_config(&config, 5, false);
    }

    pub fn test_with_config(
        config: &MinaTransportConfig,
        timeout_secs: u64,
        success_expected: bool,
    ) {
        async_std::task::block_on(async move {
            let (transport, peer_id) = {
                let builder = MinaTransportBuilder::new()
                    .with_config(config)
                    .with_timeout(Duration::from_secs(timeout_secs));
                let tcp = TcpConfig::new().nodelay(true);
                builder.build(tcp)
            };
            let dest: Multiaddr = DEST.parse().unwrap();
            let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
            let mut swarm = Swarm::new(transport, behaviour, peer_id);
            swarm.dial(dest).unwrap();
            let mut success = false;
            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Listening on {:?}", address)
                    }
                    SwarmEvent::Behaviour(event) => {
                        println!("{:?}", event);
                        event.result.unwrap();
                        success = true;
                        break;
                    }
                    _ => {
                        println!("Other");
                        break;
                    }
                }
            }
            assert_eq!(success, success_expected);
        });
    }
}
