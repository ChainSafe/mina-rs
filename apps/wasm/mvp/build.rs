// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use walkdir::WalkDir;

const PROTO_DIR: &str = "proto";
const CARGO_OUT_DIR: &str = "protobuf";

fn main() -> anyhow::Result<()> {
    protobuf_codegen::Codegen::new()
        .pure()
        .cargo_out_dir(CARGO_OUT_DIR)
        .inputs(get_inputs()?.as_slice())
        .include("proto")
        .run()?;
    Ok(())
}

fn get_inputs() -> anyhow::Result<Vec<String>> {
    let mut inputs = Vec::new();
    for entry in WalkDir::new(PROTO_DIR).into_iter().flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "proto" {
                    inputs.push(path.display().to_string());
                }
            }
        }
    }
    Ok(inputs)
}
