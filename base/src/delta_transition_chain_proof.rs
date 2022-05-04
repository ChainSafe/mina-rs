// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use derive_more::From;
use mina_crypto::hash::*;
use mina_serialization_types_macros::AutoFrom;

/// Proof that the block was produced within the allotted slot time
#[derive(Clone, Debug, Default, PartialEq, From, AutoFrom)]
#[auto_from(mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof)]
pub struct DeltaTransitionChainProof(pub StateHash, pub Vec<StateHash>);
