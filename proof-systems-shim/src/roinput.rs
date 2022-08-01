// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Module contains the implementation of chunked Random Oracle input

use ark_ff::{fields::PrimeField, BigInteger, BigInteger256, Zero};
use bitvec::prelude::*;
use mina_hasher::{Fp, ROInput};
use o1_utils::FieldHelpers;

/// Trait that converts a struct to [ChunkedROInput]
pub trait ToChunkedROInput {
    fn to_chunked_roinput(&self) -> ChunkedROInput;

    fn roinput(&self) -> ROInput {
        self.to_chunked_roinput().into()
    }
}

/// Chunked Random Oracle input
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ChunkedROInput {
    pub fields: Vec<Fp>,
    pub packed: Vec<(Fp, u32)>,
}

impl ChunkedROInput {
    /// Create a new empty random oracle input
    pub fn new() -> Self {
        Default::default()
    }

    /// Append another random oracle input
    pub fn append(mut self, other: Self) -> Self {
        self.fields.extend(other.fields.into_iter());
        self.packed.extend(other.packed.into_iter());
        self
    }

    /// Append [ToChunkedROInput]
    pub fn append_chunked(self, other: &impl ToChunkedROInput) -> Self {
        self.append(other.to_chunked_roinput())
    }

    /// Append a base field element
    pub fn append_field(mut self, f: Fp) -> Self {
        self.fields.push(f);
        self
    }

    /// Append a base field element
    pub fn append_packed(mut self, f: Fp, max_bits: u32) -> Self {
        self.packed.push((f, max_bits));
        self
    }

    /// Append a single bit
    pub fn append_bool(self, b: bool) -> Self {
        let f = {
            let mut bits = BitVec::with_capacity(1);
            bits.push(b);
            Self::bits_to_fp_unsafe(bits)
        };
        self.append_packed(f, 1)
    }

    /// Append bytes
    // pub fn append_bytes(mut self, bytes: &[u8]) -> Self {
    //     let f = {
    //         let bits = bytes.as_bits::<Lsb0>().to_bitvec();
    //         Self::bits_to_fp_unsafe(bits)
    //     };
    //     self.append_packed(f, size)
    // }

    /// Append a 32-bit unsigned integer
    pub fn append_u32(self, x: u32) -> Self {
        let f = {
            let bits = x.to_le_bytes().as_bits::<Lsb0>().to_bitvec();
            Self::bits_to_fp_unsafe(bits)
        };
        self.append_packed(f, u32::BITS)
    }

    /// Append a 64-bit unsigned integer
    pub fn append_u64(self, x: u64) -> Self {
        let f = {
            let bits = x.to_le_bytes().as_bits::<Lsb0>().to_bitvec();
            Self::bits_to_fp_unsafe(bits)
        };
        self.append_packed(f, u64::BITS)
    }

    /// Serialize random oracle input to vector of base field elements
    pub fn into_fields(self) -> Vec<Fp> {
        fn shl(f: Fp, n: u32) -> Fp {
            if n == 0 {
                f
            } else {
                let mut big: BigInteger256 = f.into();
                big.muln(n);
                big.into()
            }
        }
        let size_in_bits = Fp::size_in_bits() as u32;
        let mut fields = self.fields;
        let mut acc_bits = 0;
        let mut sum = Fp::zero();
        for (f, n_bits) in self.packed.into_iter() {
            if acc_bits + n_bits < size_in_bits {
                sum = shl(sum, n_bits) + f;
                acc_bits += n_bits;
            } else {
                fields.push(sum);
                acc_bits = 0;
                sum = Fp::zero();
                // acc_bits = n_bits;
                // sum = f;
            }
        }
        if acc_bits > 0 {
            fields.push(sum);
        }

        fields
    }

    /// Convert [BitVec] to [Fp]
    /// Note this is a temparory solution before chunked roinput is
    /// supported in proof-systems, use [anyhow::Result] for convinience
    pub fn bits_to_fp(mut bits: BitVec<u8>) -> anyhow::Result<Fp> {
        let size_in_bits = Fp::size_in_bits();
        if bits.len() > size_in_bits {
            anyhow::bail!("Input should not be greater than {size_in_bits} bits")
        } else {
            bits.resize(size_in_bits, false);
            Ok(Fp::from_bytes(&bits.into_vec())?)
        }
    }

    /// Convert [BitVec] to [Fp], panics when any error occurs
    pub fn bits_to_fp_unsafe(bits: BitVec<u8>) -> Fp {
        Self::bits_to_fp(bits).expect("Failed to create base field element")
    }
}

impl From<ChunkedROInput> for ROInput {
    fn from(i: ChunkedROInput) -> Self {
        let mut roi = ROInput::new();
        for f in i.into_fields() {
            roi.append_field(f);
        }
        roi
    }
}
