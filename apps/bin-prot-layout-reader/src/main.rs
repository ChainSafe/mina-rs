// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use env_logger::Env;
use log::info;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Input layout JSON file
    #[structopt(parse(from_os_str))]
    layout: PathBuf,

    /// Input binary file
    #[structopt(parse(from_os_str))]
    binary: PathBuf,

    /// Output file, stdout if not present
    #[structopt(long, short, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let opt = Opt::from_args();

    info!("Reading layout, please wait. This may take a minute or two...");

    // read the layout file, ensure it can be read as JSON
    let layout_file = File::open(&opt.layout)
        .with_context(|| format!("Could not open layout file to read: {:?}", opt.layout))?;
    let mut json_deserializer = serde_json::Deserializer::from_reader(layout_file);
    // need to use the disable_recursion_limit hack because these can be HUGE!
    json_deserializer.disable_recursion_limit();
    let json_deserializer = serde_stacker::Deserializer::new(&mut json_deserializer);
    let layout = bin_prot::Layout::deserialize(json_deserializer)
        .context("Failed to deserialize layout JSON")?;

    // Use this layout to read the binary
    // binary file could be either actual binary encoded bin_prot
    // OR utf-8 encoded hex string representation of the binary
    let binary_file = File::open(&opt.binary)
        .with_context(|| format!("Could not open binary file to read: {:?}", opt.binary))?;
    let mut reader = BufReader::new(binary_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    // How to know which one to use? Try and decode hex first and if that fails
    // fallback to binary interpretation
    let bytes = if let Ok(hex_bytes) = hex::decode(&buffer) {
        info!("Identified HEX encoded string input");
        hex_bytes
    } else {
        info!("Interpreting binary as raw bytes");
        buffer
    };

    // either way decode using the layout from above
    let mut de = bin_prot::Deserializer::from_reader(&bytes[..]).with_layout(&layout.bin_prot_rule);
    let result: bin_prot::Value = Deserialize::deserialize(&mut de)
        .context("Failed to deserialize binary file with given layout")?;

    // pretty print the result (or write to a file)
    if let Some(out_path) = opt.output {
        let mut out_file = File::create(&out_path)
            .with_context(|| format!("Could not create output file: {:?}", &out_path))?;
        write!(out_file, "{:#?}", result)
            .with_context(|| format!("Could not write to output file: {:?}", &out_path))?;
    } else {
        println!("{:#?}", result);
    }
    Ok(())
}
