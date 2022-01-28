// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use libp2p::{
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;

#[allow(dead_code)]
pub async fn passsive_discovery() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random PeerId.
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Create a transport.
    let transport = libp2p::development_transport(id_keys).await?;

    // Create an MDNS network behaviour.
    let behaviour = Mdns::new(MdnsConfig::default()).await?;

    // Create a Swarm that establishes connections through the given transport.
    // Note that the MDNS behaviour itself will not actually inititiate any connections,
    // as it only uses UDP.
    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {}
        }
    }
}
