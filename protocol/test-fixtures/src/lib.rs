// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]

use bin_prot::{BinProtRule, Deserializer};
use lazy_static::lazy_static;
use mina_serialization_types::v1::ExternalTransitionV1;
use serde::Deserialize;
use std::{borrow::Borrow, collections::HashMap};

// FIXME: Move layouts into this crate?
pub const BLOCK_LAYOUT: &str = include_str!("../../layouts/external_transition.json");

lazy_static! {
    pub static ref BLOCK_RULE: BinProtRule = {
        let mut deserializer = serde_json::Deserializer::from_str(BLOCK_LAYOUT);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        bin_prot::Layout::deserialize(deserializer)
            .unwrap()
            .bin_prot_rule
    };
    pub static ref GENESIS_BLOCK_MAINNET_JSON: serde_json::Value = serde_json::from_slice(include_bytes!("data/genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.json")).unwrap();
    pub static ref GENESIS_BLOCK_MAINNET: BlockFixture = load_test_block_hex("genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex", include_str!("data/genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex"));
    // FIXME: Update this with real devnet genesis block
    pub static ref GENESIS_BLOCK_DEVNET: BlockFixture = load_test_block_hex("genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex", include_str!("data/genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex"));
    pub static ref TEST_BLOCKS: HashMap<String, BlockFixture> = load_test_blocks!(
        "data/block1"
        "data/3NK3P5bJHhqR7xkZBquGGfq3sERUeXNYNma5YXRMjgCNsTJRZpgL.hex"
        "data/3NK6nkk9t23KNHTZ92M77ebpv1nzvFwQLow1DHS4eDNa2bRhtsPd.hex"
        "data/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex"
        "data/3NKf5nZYFZ4qMe1VyysbsnzgA1pm4i93NBt6ytNC7CdU2QbmdRZC.hex"
        "data/3NKCdqMnTzqxLhpG218eKZSUxB1GMqdiiBjbWXrQVrVzz8mTNFrK.hex"
        "data/3NKEWdhLxBuPboanSMXgNxudXqowm9gLNzhyfQMUBM2L52wSjb6z.hex"
        "data/3NLJmFT3nvnatDEZfqZzJ7k9fJYApoe1SPQVUggS24ViZT7z8aNA.hex"
        "data/3NLRkk5T3Gaf8ZPXgoXatdrtpN3hUdzWPWXbrqMo4jVsi3jkiGE5.hex"
        "data/3NLvrNK6rmWnxEkGZo1y4KYjsSTcgVx7gwen2aR2kTWmRDTNoSu8.hex"
        "data/3NK9fHpzfPWhuxFhQ9Dau1X1JWtstB6kGC4xrurSPU1kctMCsU9U.hex"
        "data/3NKapQX5Qe8f4BEZGWxVSWKQvKNnkvPXNLq5KDHCV1qoPzV5Y3Wu.hex"
        "data/3NKjZ5fjms6BMaH4aq7DopPGyMY7PbG6vhRsX5XnYRxih8i9G7dj.hex"
    );
    // Note that GENESIS_BLOCK_MAINNET_JSON has a different json format, so it's not included here
    pub static ref JSON_TEST_BLOCKS: HashMap<String, serde_json::Value> = load_json_test_blocks!(
        "data/mainnet-117896-3NKrv92FYZFHRNUJxiP7VGeRx3MeDY2iffFjUWXTPoXJorsS63ba.json"
        "data/mainnet-117896-3NKjZ5fjms6BMaH4aq7DopPGyMY7PbG6vhRsX5XnYRxih8i9G7dj.json"
        "data/mainnet-116121-3NK6myZRzc3GvS5iydv88on2XTEU2btYrjMVkgtbuoeXASRipSa6.json"
        "data/mainnet-77749-3NK3P5bJHhqR7xkZBquGGfq3sERUeXNYNma5YXRMjgCNsTJRZpgL.json"
        "data/mainnet-77748-3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.json"
        "data/mainnet-113267-3NLenrog9wkiJMoA774T9VraqSUGhCuhbDLj3JKbEzomNdjr78G8.json"
        "data/mainnet-147571-3NKwrze6FvGQCCF6L7Q2JLvwgnsm56hwSny9kUyjbSUr8oqu1MGp.json"
        "data/mainnet-149909-3NLCeY7UwgCryuvk3Wevm9ndMDvWAMjwGBfBJS12MqL1QoTQWEWt.json"
        "data/mainnet-113267-3NKtqqstB6h8SVNQCtspFisjUwCTqoQ6cC1KGvb6kx6n2dqKkiZS.json"
    );
}

pub const VERIFICATION_KEY: &[u8] = include_bytes!(
    "data/vk-wrap-wrap-verification-key-blockchain-snark-d3623dbfa42f563e40cd5f2d032ad91f"
);

pub struct BlockFixture {
    pub bytes: Vec<u8>,
    pub value: bin_prot::Value,
    pub block_name: &'static str,
}

impl BlockFixture {
    pub fn external_transitionv1(&self) -> anyhow::Result<ExternalTransitionV1> {
        Ok(bin_prot::from_reader_strict(self.bytes.as_slice())?)
    }
}

fn load_test_block(block_name: &'static str, bytes: &'static [u8]) -> BlockFixture {
    let mut de = Deserializer::from_reader(bytes).with_layout(&BLOCK_RULE);
    match Deserialize::deserialize(&mut de) {
        Ok(value) => BlockFixture {
            bytes: bytes.into(),
            value,
            block_name,
        },
        Err(_) => load_test_block_hex(
            block_name,
            String::from_utf8(bytes.into())
                .expect("Failed to decode hex encoded block")
                .borrow(),
        ),
    }
}

fn load_test_block_hex(block_name: &'static str, hex_str: &str) -> BlockFixture {
    let bytes = hex::decode(hex_str).expect("Failed to decode hex encoded block");
    let mut de = Deserializer::from_reader(bytes.as_slice()).with_layout(&BLOCK_RULE);
    let value = Deserialize::deserialize(&mut de).expect("Failed to deserialize test block");
    BlockFixture {
        bytes,
        value,
        block_name,
    }
}

#[macro_export]
macro_rules! load_test_blocks {
    ( $( $lt:literal $(,)?) * ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                let file_name = $lt.split('/').last().unwrap().into();
                let bytes = include_bytes!($lt);
                let block = load_test_block($lt, bytes);
                temp_map.insert(file_name, block);
            )*
            temp_map
        }
    };
}

#[macro_export]
macro_rules! load_json_test_blocks {
    ( $( $lt:literal $(,)?) * ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                let file_name = $lt.split('/').last().unwrap().into();
                let mut block: serde_json::Value = serde_json::from_slice(include_bytes!($lt)).map_err(|err|format!("Errer loading {}: {err}", $lt)).unwrap();
                // Remove scheduled_time field as it's not part of block
                if let Some(block_mut) = block.as_object_mut() {
                    block_mut.remove("scheduled_time");
                }
                temp_map.insert(file_name, block);
            )*
            temp_map
        }
    };
}
