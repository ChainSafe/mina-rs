#! /bin/bash

set -e

# This script runs the two bin prot checker implementations in Rust and Ocaml (from O1 labs)
# and does a roundtrip serialization and deserialization test for the supported types
# in the serde-bin-prot repository.

# build rust bin prot checker
cargo build --bin bin_prot_checker
# build ocaml bin prot checker
# TODO: invoke a script to build bin_prot_checker.exe locally

allTests=("nat0" "bool" "int" "int32" "int64" "enum" "record" "variant")

for test in ${allTests[@]}; do
    ## Serialize from ocaml and deserialize from Rust
    ./bin_prot_checker.exe serialize --path ./tests/payload/test.bin --test $test
    ../target/debug/bin_prot_checker --test $test --path ./tests/payload/test.bin deserialize 

    # Serialize from Rust and deserialize from ocaml
    ../target/debug/bin_prot_checker --test $test --path ./tests/payload/test.bin serialize
    ./bin_prot_checker.exe deserialize --path ./tests/payload/test.bin --test $test
done
