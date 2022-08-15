// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

pub mod js_exports;
mod js_ffi;
use js_ffi::*;
mod logger;
mod pb;
mod utils;
use libp2p::{
    core::ProtocolName,
    futures::{io::BufReader, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, StreamExt},
    request_response::{
        ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
        RequestResponseEvent,
    },
    swarm::{NetworkBehaviourEventProcess, SwarmBuilder, SwarmEvent},
    Multiaddr, NetworkBehaviour,
};
use mina_network::p2p::{TransportBuilder, MAINNET_CONFIG};
use pb::{requests::ConnectRequest, responses::CommonResponse};
use std::{io, time::Duration};
use utils::*;
mod consts;
mod event_emitter;
mod frontier;

async fn connect_async(request: &ConnectRequest) -> anyhow::Result<CommonResponse> {
    let (transport, peer_id) = {
        let builder = TransportBuilder::default()
            .with_config(&MAINNET_CONFIG)
            .with_timeout(Duration::from_secs(consts::TIMEOUT_SECS));
        builder.build()?
    };

    let parsed_addr: Multiaddr = request.address.parse().unwrap();
    log::info!(
        "[WASM] Connecting to proxy server via ws {} ... ",
        request.address
    );
    let mut swarm = {
        let behaviour = NodeStatusBehaviour::new().await.unwrap();
        SwarmBuilder::new(transport, behaviour, peer_id).build()
    };
    match swarm.dial(parsed_addr) {
        Ok(_) => {
            log::info!("[WASM] dial ok");
            loop {
                if let SwarmEvent::ConnectionEstablished { peer_id, .. } =
                    swarm.select_next_some().await
                {
                    log::info!("[WASM] Connected to {}", peer_id);
                    swarm
                        .behaviour_mut()
                        .request_response
                        .send_request(&peer_id, NodeStatusRequest);
                }
            }
        }
        Err(e) => log::error!("[WASM] Fail to dail: {}", e),
    }
    Ok({
        let mut r = CommonResponse::new();
        r.success = true;
        r
    })
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
        log::info!("[WASM] RequestResponseEvent: {:?}", event);
    }
}

#[derive(Debug, Clone)]
struct NodeStatusProtocol;

impl ProtocolName for NodeStatusProtocol {
    fn protocol_name(&self) -> &[u8] {
        // b"/mina/node-status"
        b"/mina-proxy/node-status"
    }
}

#[derive(Clone)]
struct NodeStatusCodec;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeStatusRequest;

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeStatusResponse(String);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct PeerStatusJson {
    pub connected: bool,
    pub peer_id: String,
    pub sync_status: String,
    pub protocol_state_hash: String,
    pub git_commit: String,
    pub uptime_minutes: i64,
}

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
        log::info!("[WASM] read_request");
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
        log::info!("[WASM] read_response: begin loop");
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(n) => {
                    let line = line.trim();
                    if !line.is_empty() {
                        match base64::decode(line) {
                            Ok(bytes) => {
                                if let Ok(status) = serde_json::from_slice::<PeerStatusJson>(&bytes)
                                {
                                    log::info!("[WASM] read_response: {:?}", status);
                                    let status_pb = {
                                        let mut s = pb::messages::PeerStatus::new();
                                        s.connected = status.connected;
                                        s.peer_id = status.peer_id;
                                        s.sync_status = status.sync_status;
                                        s.protocol_state_hash = status.protocol_state_hash;
                                        s.git_commit = status.git_commit;
                                        s.uptime_minutes = status.uptime_minutes;
                                        s
                                    };
                                    if let Some(ee) = event_emitter::get_event_emitter() {
                                        if let Ok(u8a) = proto_msg_to_u8array(&status_pb) {
                                            ee.emit_u8a("update", &u8a);
                                        } else {
                                            log::error!("Fail to serialize node status into protobuf binary");
                                        }
                                    }
                                }
                            }
                            _ => {
                                log::info!("[WASM] read_response({}): {}", n, line);
                            }
                        }
                    }
                }
                Err(err) => {
                    log::error!("[WASM] read_response err: {:?}", err);
                    break;
                }
            }
        }
        log::info!("[WASM] read_response: end loop");
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
        log::info!("[WASM] write_request");
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
        log::info!("[WASM] write_response: {}", json);
        io.write_all(json.as_bytes()).await?;
        io.close().await?;
        Ok(())
    }
}
