#!/bin/bash

SCRIPT_DIR=$(dirname $0)

cargo update --manifest-path $SCRIPT_DIR/../Cargo.toml
cargo update --manifest-path $SCRIPT_DIR/../apps/Cargo.toml --offline
cargo update --manifest-path $SCRIPT_DIR/../apps/wasm/Cargo.toml --offline
