// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Account token symbol

use derive_more::{From, Into};
use proof_systems::mina_hasher::{Hashable, ROInput};

/// FIXME: Need to learn exactly what this is..
#[derive(Clone, Debug, From, Into)]
pub struct TokenSymbol([u8; 32]);

impl TokenSymbol {
    /// FIXME: Need to learn exactly what this is..
    pub const fn max_length() -> usize {
        6
    }

    /// FIXME: Need to learn exactly what this is..
    pub const fn num_bits() -> usize {
        8 * Self::max_length()
    }
}

impl Hashable for TokenSymbol {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();
        // FIXME: This might not be correct
        roi.append_bytes(&self.0[..Self::max_length()]);
        roi
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}
