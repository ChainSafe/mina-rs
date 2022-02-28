// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use bin_prot::encodable::BinProtEncodable;
use clap::{App, Arg};
use mina_rs_base::types::ExternalTransition;
use std::str::FromStr;

const BLOCK_BYTES: &[u8] = include_bytes!("../../../../protocol/test-fixtures/src/data/block1");

fn main() -> anyhow::Result<()> {
    let matches = App::new("block-serde-app")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .possible_values(&["cpu", "heap"])
                .required(true)
                .default_value("cpu")
                .help("Profiling mode")
                .takes_value(true),
        )
        .get_matches();
    let mode = Mode::from_str(matches.value_of("mode").unwrap()).unwrap();
    match mode {
        Mode::Cpu => cpu_profile_serialization(),
        Mode::Heap => heap_profile_serialization(),
    }?;
    Ok(())
}

fn cpu_profile_serialization() -> anyhow::Result<ExternalTransition> {
    Ok(ExternalTransition::try_decode_binprot(BLOCK_BYTES)?)
}

fn heap_profile_serialization() -> anyhow::Result<ExternalTransition> {
    use dhat::{Dhat, DhatAlloc};

    #[global_allocator]
    static ALLOCATOR: DhatAlloc = DhatAlloc;
    let _dhat = Dhat::start_heap_profiling();

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
