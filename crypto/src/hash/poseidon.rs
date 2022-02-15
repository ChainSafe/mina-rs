// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use ark_ff::{biginteger::BigInteger256, FromBytes};
use mina_curves::pasta::*;
use oracle::poseidon::*;

/// Converts prefix string into scalar type of vesta elliptic curve
/// Note that Fp is the base field type of pallas elliptic curve and
/// the scalar field type of vesta elliptic curve
pub fn prefix_to_field(s: &[u8]) -> Result<Fp, std::io::Error> {
    const LEN: usize = 32;
    let mut bytes = [0_u8; LEN];
    for (i, &b) in s.iter().enumerate().take(LEN) {
        bytes[i] = b;
    }

    let big = BigInteger256::read(bytes.as_slice())?;
    Ok(big.into())
}

/// Gets poseidon hash state from prefix string
pub fn salt(s: &[u8]) -> Result<Vec<Fp>, std::io::Error> {
    let f = prefix_to_field(s)?;
    let mut hash =
        ArithmeticSponge::<Fp, PlonkSpongeConstantsBasic>::new(oracle::pasta::fp::params());
    hash.absorb(&[f]);
    hash.squeeze();
    Ok(hash.state)
}

/// Calculates poseidon hash of given fields with init state
/// This hash algorithm is used for implementing mina merkle tree hasher and merger
pub fn hash(init_state: Vec<Fp>, fields: &[Fp]) -> Fp {
    let mut hash =
        ArithmeticSponge::<Fp, PlonkSpongeConstantsBasic>::new(oracle::pasta::fp::params());
    hash.state = init_state;
    hash.absorb(fields);
    hash.squeeze()
}
