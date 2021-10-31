// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::{BinProtRule, Deserializer};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{collections::HashMap, include_bytes, include_str};

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
    );
}

pub struct BlockFixture {
    pub bytes: &'static [u8],
    pub value: bin_prot::Value,
}

fn load_test_block(bytes: &'static [u8]) -> BlockFixture {
    let mut de = Deserializer::from_reader(bytes).with_layout(&BLOCK_RULE);
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
