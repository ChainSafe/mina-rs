# Layouts

This folder contains layout files which describe how types from the OCaml Mina implementation are serialized to binary via Bin-prot.

These layouts can be used by the Rust bin-prot crate to deserialize from binary into loosely-typed values (similar to serde_json::Value) as the layouts provide the information usually provided by the destination type.

Large layouts are minified as they can be huge (>200MB)
