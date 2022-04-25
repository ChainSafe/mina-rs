// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// This trait annotates a given type its corresponding bin-prot serialization type,
pub trait BinProtSerializationType: Sized {
    /// The corresponding serialization type for bin-prot format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type T: From<Self> + Into<Self>;
}

/// This trait annotates a given type its corresponding json serialization type,
/// and provide utility functions to easily convert between them
pub trait JsonSerializationType: Sized {
    /// The corresponding serialization type for json format
    /// Self type can be used here to indicate no special convertion is needed
    /// TODO: Add default value when the feature lands on stable rust
    type T: From<Self> + Into<Self>;
}
