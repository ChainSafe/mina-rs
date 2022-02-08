// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;
use std::time::Duration;

async fn create_swarm(config: MdnsConfig) -> Result<Swarm<Mdns>, Box<dyn Error>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    let transport = libp2p::development_transport(id_keys).await?;
    let behaviour = Mdns::new(config).await?;
    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    Ok(swarm)
}

async fn run_discovery_test(config: MdnsConfig) -> Result<(), Box<dyn Error>> {
    env_logger::try_init().ok();
    let mut a = create_swarm(config.clone()).await?;
    let mut b = create_swarm(config).await?;
    let mut discovered_a = false;
    let mut discovered_b = false;
    loop {
        futures::select! {
            ev = a.select_next_some() => match ev {
                SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                    for (peer, _addr) in peers {
                        if peer == *b.local_peer_id() {
                            if discovered_a {
                                return Ok(());
                            } else {
                                discovered_b = true;
                            }
                        }
                    }
                }
                _ => {}
            },
            ev = b.select_next_some() => match ev {
                SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                    for (peer, _addr) in peers {
                        if peer == *a.local_peer_id() {
                            if discovered_b {
                                return Ok(());
                            } else {
                                discovered_a = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[async_std::test]
async fn test_discovery_async_std_ipv4() -> Result<(), Box<dyn Error>> {
    run_discovery_test(MdnsConfig::default()).await
}

#[tokio::test]
async fn test_discovery_tokio_ipv4() -> Result<(), Box<dyn Error>> {
    run_discovery_test(MdnsConfig::default()).await
}

#[async_std::test]
async fn test_discovery_async_std_ipv6() -> Result<(), Box<dyn Error>> {
    let config = MdnsConfig {
        enable_ipv6: true,
        ..Default::default()
    };
    run_discovery_test(config).await
}

#[tokio::test]
async fn test_discovery_tokio_ipv6() -> Result<(), Box<dyn Error>> {
    let config = MdnsConfig {
        enable_ipv6: true,
        ..Default::default()
    };
    run_discovery_test(config).await
}

async fn run_peer_expiration_test(config: MdnsConfig) -> Result<(), Box<dyn Error>> {
    let mut a = create_swarm(config.clone()).await?;
    let mut b = create_swarm(config).await?;

    loop {
        futures::select! {
            ev = a.select_next_some() => match ev {
                SwarmEvent::Behaviour(MdnsEvent::Expired(peers)) => {
                    for (peer, _addr) in peers {
                        if peer == *b.local_peer_id() {
                            return Ok(());
                        }
                    }
                }
                _ => {}
            },
            ev = b.select_next_some() => match ev {
                SwarmEvent::Behaviour(MdnsEvent::Expired(peers)) => {
                    for (peer, _addr) in peers {
                        if peer == *a.local_peer_id() {
                            return Ok(());
                        }
                    }
                }
                _ => {}
            }

        }
    }
}

#[async_std::test]
async fn test_expired_async_std() -> Result<(), Box<dyn Error>> {
    env_logger::try_init().ok();
    let config = MdnsConfig {
        ttl: Duration::from_secs(1),
        query_interval: Duration::from_secs(10),
        ..Default::default()
    };

    async_std::future::timeout(Duration::from_secs(6), run_peer_expiration_test(config))
        .await
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

#[tokio::test]
async fn test_expired_tokio() -> Result<(), Box<dyn Error>> {
    env_logger::try_init().ok();
    let config = MdnsConfig {
        ttl: Duration::from_secs(1),
        query_interval: Duration::from_secs(10),
        ..Default::default()
    };

    tokio::time::timeout(Duration::from_secs(6), run_peer_expiration_test(config))
        .await
        .unwrap()
}