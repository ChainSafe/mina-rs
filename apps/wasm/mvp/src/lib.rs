// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

// https://github.com/rustwasm/wasm-bindgen/issues/2774#issuecomment-1030747023
#![allow(clippy::unused_unit)]

mod js_ffi;
use js_ffi::*;
use libp2p::{
    core::ProtocolName,
    futures::{io::BufReader, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, StreamExt},
    identity, noise,
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
        RequestResponseEvent,
    },
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    Multiaddr, NetworkBehaviour, PeerId,
};
use mina_network::p2p::{TransportBuilder, MAINNET_CONFIG};
use std::{io, time::Duration};
use wasm_bindgen::prelude::*;

const TIMEOUT_SECS: u64 = 60;

static mut EVENT_EMITTER: Option<EventEmitter> = None;

#[wasm_bindgen]
pub fn wasm_test() -> bool {
    log_str("wasm_test");
    true
}

#[wasm_bindgen]
pub async fn wasm_test_async() -> bool {
    log_str("wasm_test_async");
    true
}

#[wasm_bindgen]
pub fn set_event_emitter(e: EventEmitter) {
    log_str("set_event_emitter");
    e.emit("update", "hello from wasm");
    unsafe { EVENT_EMITTER = Some(e) };
}

#[wasm_bindgen]
pub async fn connect(addr: String) -> bool {
    connect_async(&addr).await.unwrap()
}

async fn connect_async(addr: &str) -> anyhow::Result<bool> {
    let js_promise = js_sys::Promise::resolve(&42.into());
    let js_future: wasm_bindgen_futures::JsFuture = js_promise.into();
    let js_val = js_future.await.unwrap();
    log_string(format!("js_val: {:?}", js_val));

    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    log_string(format!("Local peer id: {:?}", peer_id));

    // Create a keypair for authenticated encryption of the transport.
    let _noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&id_keys)
        .expect("Signing libp2p-noise static DH keypair failed.");

    let mut mux_config = libp2p_mplex::MplexConfig::new();
    mux_config.set_protocol_name(b"/coda/mplex/1.0.0");
    let (transport, peer_id) = {
        let builder = TransportBuilder::default()
            .with_config(&MAINNET_CONFIG)
            .with_timeout(Duration::from_secs(TIMEOUT_SECS));
        builder.build()?
    };

    let parsed_addr: Multiaddr = addr.parse().unwrap();
    log_string(format!("Connecting to relay server via ws {} ... ", addr));
    let mut swarm = {
        let behaviour = NodeStatusBehaviour::new().await.unwrap();
        SwarmBuilder::new(transport, behaviour, peer_id).build()
    };
    match swarm.dial(parsed_addr) {
        Ok(_) => {
            log_str("dial ok");
            loop {
                if let SwarmEvent::ConnectionEstablished { peer_id, .. } =
                    swarm.select_next_some().await
                {
                    log_string(format!("Connected to {}", peer_id));
                    swarm
                        .behaviour_mut()
                        .request_response
                        .send_request(&peer_id, NodeStatusRequest);
                }
            }
        }
        Err(e) => log_string(format!("Fail to dail: {}", e)),
    }
    Ok(false)
}

fn get_event_emitter<'a>() -> Option<&'a EventEmitter> {
    unsafe {
        if let Some(ee) = &EVENT_EMITTER {
            Some(ee)
        } else {
            None
        }
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct NodeStatusBehaviour {
    request_response: RequestResponse<NodeStatusCodec>,
}

impl NodeStatusBehaviour {
    async fn new() -> anyhow::Result<Self> {
        let mut config = RequestResponseConfig::default();
        config.set_request_timeout(Duration::from_secs(60));
        Ok(Self {
            request_response: RequestResponse::new(
                NodeStatusCodec,
                std::iter::once((NodeStatusProtocol, ProtocolSupport::Full)),
                config,
            ),
        })
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<NodeStatusRequest, NodeStatusResponse>>
    for NodeStatusBehaviour
{
    fn inject_event(&mut self, event: RequestResponseEvent<NodeStatusRequest, NodeStatusResponse>) {
        log_string(format!("RequestResponseEvent: {:?}", event));
    }
}

#[derive(Debug, Clone)]
struct NodeStatusProtocol;

impl ProtocolName for NodeStatusProtocol {
    fn protocol_name(&self) -> &[u8] {
        // b"/mina/node-status"
        b"/webnode"
    }
}

#[derive(Clone)]
struct NodeStatusCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeStatusRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeStatusResponse(String);

#[async_trait::async_trait]
impl RequestResponseCodec for NodeStatusCodec {
    type Protocol = NodeStatusProtocol;
    type Request = NodeStatusRequest;
    type Response = NodeStatusResponse;

    async fn read_request<T>(
        &mut self,
        _: &Self::Protocol,
        _io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        log_str("read_request");
        Ok(NodeStatusRequest)
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut reader = BufReader::new(io);
        log_str("read_response: begin loop");
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(n) => {
                    let line = line.trim();
                    if !line.is_empty() {
                        match base64::decode(line) {
                            Ok(msg) => {
                                let decoded = String::from_utf8_lossy(&msg);
                                log_string(format!("read_response: {}", decoded));
                                if let Some(ee) = get_event_emitter() {
                                    ee.emit("update", &decoded);
                                }
                            }
                            _ => {
                                log_string(format!("read_response({}): {}", n, line));
                                if let Some(ee) = get_event_emitter() {
                                    ee.emit("log", line);
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    log_string(format!("read_response err: {:?}", err));
                    break;
                }
            }
        }
        log_str("read_response: end loop");
        Ok(NodeStatusResponse("read_response".into()))
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        _: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        log_str("write_request");
        io.close().await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        NodeStatusResponse(json): Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        log_string(format!("write_response: {}", json));
        io.write_all(json.as_bytes()).await?;
        io.close().await?;
        Ok(())
    }
}
