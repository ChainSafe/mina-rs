// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use ark_ec::AffineCurve;
use mina_curves::pasta::*;
use o1_utils::field_helpers::{FieldHelpers, FieldHelpersError};
use oracle::poseidon::*;

/// Converts prefix string into scalar type of vesta elliptic curve
/// Note that Fp is the base field type of pallas elliptic curve and
/// the scalar field type of vesta elliptic curve
pub fn prefix_to_field(
    s: &[u8],
) -> Result<<vesta::Affine as AffineCurve>::ScalarField, FieldHelpersError> {
    // Need to pad bytes into 256 bits
    // All predefined prefixes are 160 bits
    const LEN: usize = 32;
    let mut bytes = [0_u8; LEN];
    for (i, &b) in s.iter().enumerate().take(LEN) {
        bytes[i] = b;
    }

    <vesta::Affine as AffineCurve>::ScalarField::from_bytes(&bytes)
}

/// Gets poseidon hash state from prefix string
pub fn salt(
    s: &[u8],
) -> Result<Vec<<vesta::Affine as AffineCurve>::ScalarField>, FieldHelpersError> {
    let f = prefix_to_field(s)?;
    let mut hash = ArithmeticSponge::<
        <vesta::Affine as AffineCurve>::ScalarField,
        PlonkSpongeConstantsBasic,
    >::new(oracle::pasta::fp::params());
    hash.absorb(&[f]);
    hash.squeeze();
    Ok(hash.state)
}

/// Calculates poseidon hash of given fields with init state
/// This hash algorithm is used for implementing mina merkle tree hasher and merger
pub fn hash(
    init_state: Vec<<vesta::Affine as AffineCurve>::ScalarField>,
    fields: &[<vesta::Affine as AffineCurve>::ScalarField],
) -> <vesta::Affine as AffineCurve>::ScalarField {
    let mut hash = ArithmeticSponge::<
        <vesta::Affine as AffineCurve>::ScalarField,
        PlonkSpongeConstantsBasic,
    >::new(oracle::pasta::fp::params());
    hash.state = init_state;
    hash.absorb(fields);
    hash.squeeze()
}
