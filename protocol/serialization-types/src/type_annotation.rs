// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// This trait annotates a given type its different serializable types,
/// and provide utility functions to easily convert between them
pub trait TypeAnnotation {
    /// The corresponding serializable type for bin-prot format
    type BinProtType;
    /// The corresponding serializable type for json format
    type JsonType;
}
