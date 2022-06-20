// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_rs_base::types::ExternalTransition;
use mina_rs_base::JsonSerializationType;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::path::Path;

// To read query response from graphql api (https://graphql.minaexplorer.com/)
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub data: Blocks,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Blocks {
    pub blocks: Vec<BlockInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    #[serde(rename = "blockHeight")]
    pub block_height: usize,
    #[serde(rename = "stateHash")]
    pub state_hash: String,
}

impl std::fmt::Display for BlockInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.block_height, self.state_hash)
    }
}

pub async fn check_block(network: &str, block: BlockInfo) {
    // Check for previously failed block and delete it to check again
    let local_failed_block = format!("output/failed/{}-{}.json", network, block);
    if Path::new(&local_failed_block).exists() {
        match std::fs::remove_file(&local_failed_block) {
            Ok(_) => info!("Deleted {}", local_failed_block),
            Err(err) => error!("Failed to delete {}: {}", local_failed_block, err),
        }
    }
    // Check for previously succeeded block
    let local_succeeded_block = format!("output/succeeded/{}-{}.json", network, block);
    if !Path::new(&local_succeeded_block).exists() {
        info!("Checking: {}", block);
        let block_url = format!(
            "https://storage.googleapis.com/mina_network_block_data/{}-{}.json",
            network, block
        );
        // Retrieve block json from mainnet json dump
        match retrieve_block_json(&block_url).await {
            Ok(block_json) => match parse_block_json(&block_json) {
                Ok(_et) => {
                    write_json("succeeded", network, &block, &block_json);
                }
                Err(err) => {
                    write_json("failed", network, &block, &block_json);
                    error!(
                        "Fix Me(https://storage.googleapis.com/mina_network_block_data/{}-{}.json): {}",
                        network, block, err
                    );
                }
            },
            Err(err) => {
                warn!("Failed to retrieve block json{}: {}", block, err);
            }
        };
    } else {
        info!("Already checked {} ", local_succeeded_block);
    }
}

pub async fn fetch_block_info(n: &str) -> Result<QueryResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "query Fetch {{
            blocks(limit: {}, sortBy: RECEIVEDTIME_DESC) {{
                blockHeight
                stateHash
            }}
        }}",
        n
    );
    let res = client
        .post("https://graphql.minaexplorer.com/")
        // Below query retrieves blockHeight and stateHash of 10 latest blocks
        // To learn more about graphql query refer to https://docs.minaprotocol.com/en/developers/graphql-api
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;
    res.json::<QueryResponse>().await
}

pub async fn retrieve_block_json(
    block_url: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let block_txt = reqwest::get(block_url).await?.text().await?;
    let mut block_json: serde_json::Value = serde_json::from_str(&block_txt)?;
    // Remove scheduled_time field as it's not part of block
    if let Some(block_mut) = block_json.as_object_mut() {
        block_mut.remove("scheduled_time");
    } else {
        error!("Failed to parse json value as object: {}", block_url)
    }
    Ok(block_json)
}

pub fn parse_block_json(block_json: &serde_json::Value) -> Result<ExternalTransition, Error> {
    let json_value: <ExternalTransition as JsonSerializationType>::T =
        serde_json::from_value(block_json.to_owned())?;
    Ok(json_value.into())
}

pub fn write_json(op_type: &str, network: &str, block: &BlockInfo, block_json: &serde_json::Value) {
    let output_path = format!("./output/{}/{}-{}.json", op_type, network, block);
    // Save the JSON structure into file path: ./output/succeeded/
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&block_json).unwrap(),
    )
    .unwrap();
}
