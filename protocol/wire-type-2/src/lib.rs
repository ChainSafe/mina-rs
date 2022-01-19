// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![deny(missing_docs)]

//! # Wire Type
//!
//!  `wire-type` exports a trait to annotate internal types in Mina-rs that must be communicated between nodes
//!  (which may be running differnet versions/implementations of the protocol)
//!  
//!  To ensure binary compatibility with the existing implementation, types from the Rust implementation are typically converted
//!  to another type (from the network-types crate) that, once serialized, produces a bin-prot representation identical to the OCaml code.
//!  
//!  This trait links internal types to their network representation and hence requires implementations of From and Into.
//!  
//!  Note this trait only needs to be implemented for the top level types that are to be sent and received between nodes
//!  

use serde::{Deserialize, Serialize};

/// Types implementing WireType provide an ascociated wire type. Conversions are automatically implemented from To/From impls
pub trait WireType<'a>:
    Into<Self::WireType>
{
    /// Associated wire type for this type
    type WireType: Serialize + Deserialize<'a> + Into<Self>;

    /// Convert this type to its wire format
    fn to_wire_type(self) -> Self::WireType {
        self.into()
    }

    /// Convert this type from its wire format
    fn from_wire_type(t: Self::WireType) -> Self {
        t.into()
    }
}
