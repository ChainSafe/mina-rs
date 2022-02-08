// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::p2p::constants::DEST;
use libp2p::{
    futures::StreamExt,
    gossipsub::{Gossipsub, GossipsubEvent},
    identify::{Identify, IdentifyEvent},
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    ping,
    swarm::{NetworkBehaviourEventProcess, Swarm, SwarmEvent},
    NetworkBehaviour, PeerId,
};
use std::error::Error;

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct NetworkBehaviour {
    mdns: Mdns,
    gossipsub: Gossipsub,
    identify: Identify,
    ping: ping::Behaviour,
}

/// Called when `mdns` produces an event.
impl NetworkBehaviourEventProcess<MdnsEvent> for NetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                println!("discovered {} {}", peer_id, multiaddr);
            }
        }
    }
}

/// Called when `gossipsub` produces an event.
impl NetworkBehaviourEventProcess<GossipsubEvent> for NetworkBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => println!(
                "Got message: {} with id: {} from peer: {:?}",
                String::from_utf8_lossy(&message.data),
                id,
                peer_id
            ),
            _ => {}
        }
    }
}

/// Called when `identify` produces an event.
impl NetworkBehaviourEventProcess<IdentifyEvent> for NetworkBehaviour {
    fn inject_event(&mut self, event: IdentifyEvent) {
        println!("identify: {:?}", event);
    }
}

/// Called when `ping` produces an event.
impl NetworkBehaviourEventProcess<ping::Event> for NetworkBehaviour {
    fn inject_event(&mut self, event: ping::Event) {
        match event {
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Ping { rtt }),
            } => {
                println!(
                    "ping: rtt to {} is {} ms",
                    peer.to_base58(),
                    rtt.as_millis()
                );
            }
            ping::Event {
                peer,
                result: Result::Ok(ping::Success::Pong),
            } => {
                println!("ping: pong from {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Timeout),
            } => {
                println!("ping: timeout to {}", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Unsupported),
            } => {
                println!("ping: {} does not support ping protocol", peer.to_base58());
            }
            ping::Event {
                peer,
                result: Result::Err(ping::Failure::Other { error }),
            } => {
                println!("ping: ping::Failure with {}: {}", peer.to_base58(), error);
            }
        }
    }
}

#[allow(dead_code)]
/// mdns passive discovery
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
    swarm.listen_on(DEST.parse()?)?;
    let mut discovery = false;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
                discovery = true;
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
                discovery = false;
            }
            _ => {}
        }
    }
}
