// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Implementation of the Unsigned_Extended types used in Mina-ocaml
//! 
//! These are handled slightly differently than regular unsigned integers when serialized to bin-prot
//! 

use versioned::Versioned;
use serde::{Serialize, Deserialize};

// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina

/// u32 wrapped in 1 version byte
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ExtendedU32(#[serde(with = "extended_u32")] pub u32);

impl TryFrom<i32> for ExtendedU32 {
	type Error = core::num::TryFromIntError;
	
	fn try_from(t: i32) -> Result<Self, Self::Error> {
		Ok(Self(t.try_into()?))
	}
}

impl From<u32> for ExtendedU32 {
	fn from(t: u32) -> Self { Self(t) }
}

mod extended_u32 {
	use serde::{de, Serializer, Deserializer};
	use core::fmt;
	
	// In the OCaml codebase the extended unsigned integer types are serialized and deserialized
	// by first doing a truncated conversion to a signed integer then serialized using the variable 
	// length bin_prot encoding

	pub fn serialize<S>(v: &u32, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        serializer.serialize_i32((*v).try_into().unwrap())
	}

	struct ExtendedU32Visitor;

	impl<'de> de::Visitor<'de> for ExtendedU32Visitor {
	    type Value = u32;

	    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
	        formatter.write_str("an integer between 0 and 2^31 that can be safely cast to an unsigned")
	    }

	    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
	    where
	        E: de::Error,
	    {
	    	match value.try_into() {
	    		Ok(v) => Ok(v),
	    		Err(e) => {
	    			println!("Cannot deserialize value {}:  {}", value, e);
	    			panic!()
	    		}
	    	}
	    }
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error> where
        D: Deserializer<'de> {
        	deserializer.deserialize_i32(ExtendedU32Visitor)
	}
}

/// u64 wrapped in 1 version byte
pub type ExtendedU32V1 = Versioned<Versioned<ExtendedU32, 1>, 1>;

/// u64 wrapped in 1 version byte
pub type ExtendedU64 = Versioned<u64, 1>;

/// u64 wrapped in 2 version bytes
pub type ExtendedU64_2 = Versioned<ExtendedU64, 1>;

/// u64 wrapped in 3 version bytes
pub type ExtendedU64_3 = Versioned<ExtendedU64_2, 1>;
