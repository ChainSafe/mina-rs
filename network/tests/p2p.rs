// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use libp2p::{
        futures::StreamExt,
        ping,
        swarm::{SwarmBuilder, SwarmEvent},
        Multiaddr,
    };
    use mina_network::p2p::{TransportBuilder, TransportConfig, MAINNET_CONFIG};

    const DEST: &str =
        "/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb";

    #[ignore = "flacky"]
    #[tokio::test]
    pub async fn mainnet() -> anyhow::Result<()> {
        let config = MAINNET_CONFIG.clone();
        test_with_config(&config, 10, true).await?;
        Ok(())
    }

    #[tokio::test]
    pub async fn dummy_mplex() -> anyhow::Result<()> {
        let mut config = MAINNET_CONFIG.clone();
        config.mplex_protocol_name = b"dummy";
        test_with_config(&config, 5, false).await?;
        Ok(())
    }

    #[tokio::test]
    pub async fn dummy_rendezvous() -> anyhow::Result<()> {
        let mut config: TransportConfig = Default::default();
        config.rendezvous_string = b"dummy";
        test_with_config(&config, 5, false).await?;
        Ok(())
    }

    pub async fn test_with_config(
        config: &TransportConfig<'static>,
        timeout_secs: u64,
        success_expected: bool,
    ) -> anyhow::Result<()> {
        let (transport, peer_id) = {
            let builder = TransportBuilder::default()
                .with_config(config)
                .with_timeout(Duration::from_secs(timeout_secs));
            builder.build()?
        };
        let dest: Multiaddr = DEST.parse().unwrap();
        let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
        let mut swarm = SwarmBuilder::new(transport, behaviour, peer_id)
            // executor has to be explicitly set due to https://github.com/libp2p/rust-libp2p/issues/2173
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build();
        swarm.dial(dest).unwrap();
        let mut success = false;
        loop {
            match swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    // To see this in stdout
                    // run tests with 'cargo test --release --  --nocapture'
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
        Ok(())
    }
}
