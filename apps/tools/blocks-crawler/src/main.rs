// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use blocks::{check_block, fetch_block_info};
use clap::Parser;
use futures::{stream, StreamExt};
use std::fs::create_dir_all;
#[macro_use]
extern crate log;

mod blocks;

#[derive(Parser)]
struct Cli {
    #[clap(short = 'l', long = "limit")]
    limit: String,
    #[clap(short = 'c', long = "concurrency", default_value_t = 8)]
    concurrency: usize,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    // Create output folders
    create_dir_all(&"output/succeeded/").unwrap();
    create_dir_all(&"output/failed/").unwrap();

    // Read limit value from cli
    let limit = args.limit;

    // Fetch n blocks info from graphql api
    let blocks = fetch_block_info(&limit).await.unwrap();
    stream::iter(blocks.data.blocks)
        .for_each_concurrent(args.concurrency, |block| async move {
            check_block("mainnet", block).await;
        })
        .await;
}
