// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use walkdir::WalkDir;

const PROTO_DIR: &str = "proto";
const OUT_DIR: &str = "src/pb";

fn main() -> anyhow::Result<()> {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(OUT_DIR)
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
