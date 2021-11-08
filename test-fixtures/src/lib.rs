// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::{BinProtRule, Deserializer};
use lazy_static::lazy_static;
use mina_rs_base::external_transition::ExternalTransition;
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
    // FIXME: Update this with real genesis block
    pub static ref GENESIS_BLOCK: BlockFixture = load_test_block(include_bytes!("data/block1"));
    pub static ref TEST_BLOCKS: HashMap<String, BlockFixture> = load_test_blocks!(
        // "data/genesis_block"
        "data/block1"
        "data/3NK3P5bJHhqR7xkZBquGGfq3sERUeXNYNma5YXRMjgCNsTJRZpgL.hex"
        "data/3NK6nkk9t23KNHTZ92M77ebpv1nzvFwQLow1DHS4eDNa2bRhtsPd.hex"
        "data/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex"
        // FIXME: Enable failing blocks
        // "data/3NLvrNK6rmWnxEkGZo1y4KYjsSTcgVx7gwen2aR2kTWmRDTNoSu8.hex"
    );
}

pub struct BlockFixture {
    pub bytes: Vec<u8>,
    pub value: bin_prot::Value,
}

impl BlockFixture {
    pub fn external_transition(&self) -> anyhow::Result<ExternalTransition> {
        let mut de = Deserializer::from_reader(self.bytes.as_slice());
        Ok(Deserialize::deserialize(&mut de)?)
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
