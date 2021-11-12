use anyhow::Result;
use libp2p::{
    core::upgrade,
    identity, noise,
    pnet::{PnetConfig, PreSharedKey},
    tcp::TokioTcpConfig,
    websocket::WsConfig,
    PeerId, Transport,
};
use libp2p_relay::RelayConfig;
use multihash::{Blake2b256, StatefulHasher};
use std::time::Duration;

const RENDEZVOUS_STRING: &str =
    "/coda/0.0.1/5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1";
const RELAY_SERVER_ADDR: &str =
    "/ip4/127.0.0.1/tcp/43047/p2p/12D3KooWETSQx1VDh1xoq1rwAaYFzzt4KGvXnKVEquvh2m6G64Ge";
const RELAY_SERVER_WS_ADDR: &str =
    "/ip4/127.0.0.1/tcp/36795/ws/p2p/12D3KooWETSQx1VDh1xoq1rwAaYFzzt4KGvXnKVEquvh2m6G64Ge";
const MINA_PEER_ADDR: &str =
    "/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb";
// "/ip4/127.0.0.1/tcp/8302/p2p/12D3KooWKK3RpV1MWAZk3FJ5xqbVPL2BMDdUEGSfwfQoUprBNZCv";

#[tokio::main]
// #[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("Relay node address: {}", RELAY_SERVER_ADDR);
    println!("Relay node ws address: {}", RELAY_SERVER_WS_ADDR);
    println!("Mina node address: {}", MINA_PEER_ADDR);

    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    // Create a keypair for authenticated encryption of the transport.
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&id_keys)
        .expect("Signing libp2p-noise static DH keypair failed.");

    let mut hasher = Blake2b256::default();
    hasher.update(RENDEZVOUS_STRING.as_bytes());
    let hash = hasher.finalize();
    let psk = hash.as_ref();
    println!("psk: {}", hex::encode(psk));
    let mut psk_fixed: [u8; 32] = Default::default();
    psk_fixed.copy_from_slice(&psk[0..32]);
    let psk = PreSharedKey::new(psk_fixed);
    let mut mux_config = libp2p_mplex::MplexConfig::new();
    mux_config.set_protocol(b"/coda/mplex/1.0.0");
    // Create a tokio-based TCP transport use noise for authenticated
    // encryption and Mplex for multiplexing of substreams on a TCP stream.
    let transport = {
        let tcp = TokioTcpConfig::new().nodelay(true);
        let ws = WsConfig::new(tcp.clone());
        tcp.or_transport(ws)
            .and_then(move |socket, _| PnetConfig::new(psk).handshake(socket))
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
            .multiplex(mux_config)
            .boxed()
    };

    print!("Connecting to relay server via tcp ... ");
    match transport
        .clone()
        .dial(RELAY_SERVER_ADDR.parse().unwrap())
        .unwrap()
        .await
    {
        Ok(_) => {
            println!("succeeded!");
        }
        Err(e) => {
            println!("failed: {}", e);
        }
    }

    print!("Connecting to mina node via tcp ... ");
    match transport
        .clone()
        .dial(MINA_PEER_ADDR.parse().unwrap())
        .unwrap()
        .await
    {
        Ok(_) => {
            println!("succeeded!");
        }
        Err(e) => {
            println!("failed: {}", e);
        }
    }

    print!("Connecting to relay server via ws ... ");
    match transport
        .clone()
        .dial(RELAY_SERVER_WS_ADDR.parse().unwrap())
        .unwrap()
        .await
    {
        Ok(_) => {
            println!("succeeded!");
        }
        Err(e) => {
            println!("failed: {}", e);
        }
    }

    let relay_config = RelayConfig {
        connection_idle_timeout: Duration::from_secs(10 * 60),
        ..Default::default()
    };
    let (relay_wrapped_transport, _relay_behaviour) =
        libp2p_relay::new_transport_and_behaviour(relay_config, transport);
    let mina_peer_addr_via_relay = format!(
        "{}/p2p-circuit/p2p/{}",
        RELAY_SERVER_ADDR,
        MINA_PEER_ADDR.split("/").last().unwrap()
    );
    println!(
        "connecting to mina node via relay address: {} ... ",
        mina_peer_addr_via_relay
    );
    match relay_wrapped_transport
        .dial(mina_peer_addr_via_relay.parse().unwrap())
        .unwrap()
        .await
    {
        Ok(_) => {
            println!("succeeded!");
        }
        Err(e) => {
            println!("failed: {}", e);
        }
    }

    Ok(())
}
