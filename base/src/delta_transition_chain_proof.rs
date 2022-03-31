// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use derive_more::From;
use mina_crypto::hash::StateHash;


/// Proof that the block was produced within the allotted slot time
#[derive(Clone, Debug, Default, PartialEq, From)]
pub struct DeltaTransitionChainProof(pub StateHash, pub Vec<StateHash>);
