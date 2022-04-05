// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings)]

use bin_prot::{encodable::BinProtEncodable, BinProtRule, Deserializer};
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
    pub static ref GENESIS_BLOCK_MAINNET: BlockFixture = load_test_block_hex(include_str!("data/genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex"));
    // FIXME: Update this with real devnet genesis block
    pub static ref GENESIS_BLOCK_DEVNET: BlockFixture = load_test_block_hex(include_str!("data/genesis-3NKeMoncuHab5ScarV5ViyF16cJPT4taWNSaTLS64Dp67wuXigPZ.hex"));
    pub static ref TEST_BLOCKS: HashMap<String, BlockFixture> = load_test_blocks!(
        // "data/block1"
        // "data/3NK3P5bJHhqR7xkZBquGGfq3sERUeXNYNma5YXRMjgCNsTJRZpgL.hex"
        // "data/3NK6nkk9t23KNHTZ92M77ebpv1nzvFwQLow1DHS4eDNa2bRhtsPd.hex"
        // "data/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex"
        // "data/3NKf5nZYFZ4qMe1VyysbsnzgA1pm4i93NBt6ytNC7CdU2QbmdRZC.hex"
        // "data/3NKCdqMnTzqxLhpG218eKZSUxB1GMqdiiBjbWXrQVrVzz8mTNFrK.hex"
        // "data/3NKEWdhLxBuPboanSMXgNxudXqowm9gLNzhyfQMUBM2L52wSjb6z.hex"
        // "data/3NLJmFT3nvnatDEZfqZzJ7k9fJYApoe1SPQVUggS24ViZT7z8aNA.hex"
        // "data/3NLRkk5T3Gaf8ZPXgoXatdrtpN3hUdzWPWXbrqMo4jVsi3jkiGE5.hex"

        // FIXME: Enable failing blocks by implementing BinProtRule::Polyvar for loose_deserializer
        // "data/3NLvrNK6rmWnxEkGZo1y4KYjsSTcgVx7gwen2aR2kTWmRDTNoSu8.hex"
        "data/3NK9fHpzfPWhuxFhQ9Dau1X1JWtstB6kGC4xrurSPU1kctMCsU9U.hex"
        // "data/3NKapQX5Qe8f4BEZGWxVSWKQvKNnkvPXNLq5KDHCV1qoPzV5Y3Wu.hex"
    );
}

pub const VERIFICATION_KEY: &[u8] = include_bytes!(
    "data/vk-wrap-wrap-verification-key-blockchain-snark-d3623dbfa42f563e40cd5f2d032ad91f"
);

pub struct BlockFixture {
    pub bytes: Vec<u8>,
    pub value: bin_prot::Value,
}

impl BlockFixture {
    pub fn external_transitionv1(&self) -> anyhow::Result<ExternalTransitionV1> {
        Ok(ExternalTransitionV1::try_decode_binprot(
            self.bytes.as_slice(),
        )?)
    }
}

fn load_test_block(bytes: &'static [u8]) -> BlockFixture {
    let mut de = Deserializer::from_reader(bytes).with_layout(&BLOCK_RULE);
    match Deserialize::deserialize(&mut de) {
        Ok(value) => BlockFixture {
            bytes: bytes.into(),
            value,
        },
        Err(_) => load_test_block_hex(
            String::from_utf8(bytes.into())
                .expect("Failed to decode hex encoded block")
                .borrow(),
        ),
    }
}

fn load_test_block_hex(hex_str: &str) -> BlockFixture {
    let bytes = hex::decode(hex_str).expect("Failed to decode hex encoded block");
    let mut de = Deserializer::from_reader(bytes.as_slice()).with_layout(&BLOCK_RULE);
    let value = Deserialize::deserialize(&mut de).expect("Failed to deserialize test block");
    BlockFixture { bytes, value }
}

#[macro_export]
macro_rules! load_test_blocks {
    ( $( $lt:literal $(,)?) * ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                let file_name = $lt.split('/').last().unwrap().into();
                let bytes = include_bytes!($lt);
                let block = load_test_block(bytes);
                temp_map.insert(file_name, block);
            )*
            temp_map
        }
    };
}
