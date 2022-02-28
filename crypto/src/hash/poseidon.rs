// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use ark_ec::AffineCurve;
use ark_ff::{FpParameters, PrimeField};
use mina_curves::pasta::vesta;
use o1_utils::field_helpers::{FieldHelpers, FieldHelpersError};
use oracle::poseidon::*;

type FieldType = <vesta::Affine as AffineCurve>::ScalarField;

/// Converts prefix string into scalar type of vesta elliptic curve
/// Note that Fp is the base field type of pallas elliptic curve and
/// the scalar field type of vesta elliptic curve
pub fn prefix_to_field(s: &[u8]) -> Result<FieldType, FieldHelpersError> {
    // Need to pad bytes into MODULUS_BITS bits
    // All predefined prefixes are 160 bits
    const BITS: u32 = <<FieldType as PrimeField>::Params as FpParameters>::MODULUS_BITS;
    const LEN: usize = bits_to_byte_len(BITS as usize);
    let mut bytes = [0_u8; LEN];
    for (i, &b) in s.iter().enumerate().take(LEN) {
        bytes[i] = b;
    }
    FieldType::from_bytes(&bytes)
}

/// Gets poseidon hash state from prefix string
pub fn salt(s: &[u8]) -> Result<Vec<FieldType>, FieldHelpersError> {
    let f = prefix_to_field(s)?;
    let mut hash =
        ArithmeticSponge::<FieldType, PlonkSpongeConstantsBasic>::new(oracle::pasta::fp::params());
    hash.absorb(&[f]);
    hash.squeeze();
    Ok(hash.state)
}

/// Calculates poseidon hash of given fields with init state
/// This hash algorithm is used for implementing mina merkle tree hasher and merger
pub fn hash(init_state: Vec<FieldType>, fields: &[FieldType]) -> FieldType {
    let mut hash =
        ArithmeticSponge::<FieldType, PlonkSpongeConstantsBasic>::new(oracle::pasta::fp::params());
    hash.state = init_state;
    hash.absorb(fields);
    hash.squeeze()
}

const fn bits_to_byte_len(bits: usize) -> usize {
    match bits % 8 {
        0 => bits / 8,
        _ => bits / 8 + 1,
    }
}
