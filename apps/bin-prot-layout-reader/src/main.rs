// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize};
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


fn main() {
    let opt = Opt::from_args();

    println!("Reading layout, please wait. This may take a minute or two...");

    // read the layout file, ensure it can be read as JSON
    let layout_file = File::open(opt.layout).expect("Could not open layout file to read");
    let mut json_deserializer = serde_json::Deserializer::from_reader(layout_file);
    // need to use the disable_recursion_limit hack because these can be HUGE!
    json_deserializer.disable_recursion_limit();
    let json_deserializer = serde_stacker::Deserializer::new(&mut json_deserializer);
    let layout = bin_prot::Layout::deserialize(json_deserializer).expect("Failed to read layout JSON");


    // Use this layout to read the binary
    // binary file could be either actual binary encoded bin_prot 
    // OR utf-8 encoded hex string representation of the binary
    let binary_file = File::open(opt.binary).expect("Could not open binary file to read");
    let mut reader = BufReader::new(binary_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    // How to know which one to use? Try and decode hex first and if that fails 
    // fallback to binary interpretation
    let bytes = if let Ok(hex_bytes) = hex::decode(&buffer) {
        println!("Info: Identified HEX encoded string input");
        hex_bytes
    } else {
        println!("Info: Interpreting binary as raw bytes");
        buffer
    };


    // either way decode using the layout from above
    let mut de = bin_prot::Deserializer::from_reader(&bytes[..]).with_layout(&layout.bin_prot_rule);
    let result: bin_prot::Value = Deserialize::deserialize(&mut de).expect("Failed to deserialize");

    // pretty print the result (or write to a file)
    if let Some(out_path) = opt.output {
        let mut out_file = File::create(out_path).expect("Could not create output file");
        write!(out_file, "{:#?}", result).expect("Failed to write to output file");
    } else {
        println!("{:#?}", result);
    }
}
