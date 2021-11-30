// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::hash::*;

pub type DeltaTransitionChainProof = (LedgerHash, Vec<LedgerHash>);
