// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use clap::{Arg, Command};
use mina_rs_base::{types::*, *};
use std::str::FromStr;

const BLOCK_BYTES: &[u8] = include_bytes!("../../../../protocol/test-fixtures/src/data/block1");

fn main() -> anyhow::Result<()> {
    let matches = Command::new("block-serde-app")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .value_parser(["cpu", "heap"])
                .required(true)
                .default_value("cpu")
                .help("Profiling mode")
                .takes_value(true),
        )
        .get_matches();
    let mode: &String = matches.get_one("mode").unwrap();
    let mode = Mode::from_str(mode.as_str()).unwrap();
    match mode {
        Mode::Cpu => cpu_profile_serialization(),
        Mode::Heap => heap_profile_serialization(),
    }?;
    Ok(())
}

fn cpu_profile_serialization() -> anyhow::Result<ExternalTransition> {
    Ok(<ExternalTransition as BinProtSerializationType>::try_from_binprot(BLOCK_BYTES)?)
}

fn heap_profile_serialization() -> anyhow::Result<ExternalTransition> {
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;
    let _profiler = dhat::Profiler::new_heap();

    cpu_profile_serialization()
}

#[derive(Debug)]
enum Mode {
    Cpu,
    Heap,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cpu" => Ok(Mode::Cpu),
            "heap" => Ok(Mode::Heap),
            _ => Err("no match"),
        }
    }
}
