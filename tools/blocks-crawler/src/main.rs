// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_rs_base::types::ExternalTransition;
use mina_rs_base::JsonSerializationType;
use serde::{Deserialize, Serialize};
use serde_json::Error;

// To read query response from graphql api (https://graphql.minaexplorer.com/)
#[derive(Debug, Serialize, Deserialize)]
struct QueryResponse {
    data: Blocks,
}
#[derive(Debug, Serialize, Deserialize)]
struct Blocks {
    blocks: Vec<BlockInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
struct BlockInfo {
    #[serde(rename = "blockHeight")]
    block_height: usize,
    #[serde(rename = "stateHash")]
    state_hash: String,
}

#[tokio::main]
async fn main() {
    // Fetch n blocks info from graphql api
    let blocks = fetch_block_info().await.unwrap();
    for block in blocks.data.blocks {
        println!("Checking: {}-{}", block.block_height, block.state_hash);
        // Retrieve block json from mainnet json dump
        let block_json = match retrieve_block_json("mainnet", &block).await {
            Ok(val) => val,
            Err(err) => {
                println!("Failed to retrieve block json: {}", err);
                continue;
            }
        };
        match parse_block_json(block_json) {
            Ok(_et) => {} // Block parsed successfully
            Err(err) => {
                println!(
                    "Fix Me(https://storage.googleapis.com/mina_network_block_data/mainnet-{}-{}.json): {}",
                    block.block_height, block.state_hash, err
                );
                continue;
            }
        }
    }
}

async fn fetch_block_info() -> Result<QueryResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://graphql.minaexplorer.com/")
        // Below query retrieves blockHeight and stateHash of 10 latest blocks
        // To learn more about graphql query refer to https://docs.minaprotocol.com/en/developers/graphql-api 
        .json(&serde_json::json!({
            "query":"query Fetch {\n  blocks(limit: 10, sortBy: RECEIVEDTIME_DESC) {\n    blockHeight\n    stateHash\n  }\n}"
        }))
        .send()
        .await?;
    Ok(res.json::<QueryResponse>().await?)
}

async fn retrieve_block_json(
    network: &str,
    block: &BlockInfo,
) -> Result<serde_json::Value, reqwest::Error> {
    let block_url = format!(
        "https://storage.googleapis.com/mina_network_block_data/{}-{}-{}.json",
        network, block.block_height, block.state_hash
    );
    let mut block_json: serde_json::Value = reqwest::get(block_url).await?.json().await?;
    // Remove scheduled_time field as it's not part of block
    if let Some(block_mut) = block_json.as_object_mut() {
        block_mut.remove("scheduled_time");
    }
    Ok(block_json)
}

fn parse_block_json(block_json: serde_json::Value) -> Result<ExternalTransition, Error> {
    let json_value: <ExternalTransition as JsonSerializationType>::T =
        serde_json::from_value(block_json)?;
    Ok(json_value.into())
}
