// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// This trait annotates a given type its different serialization types,
/// and provide utility functions to easily convert between them
pub trait TypeAnnotation: Sized {
    /// The corresponding serialization type for bin-prot format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type BinProtType: From<Self> + Into<Self>;

    /// The corresponding serialization type for json format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type JsonType: From<Self> + Into<Self>;
}
