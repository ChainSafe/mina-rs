// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Delta transition chain proof structures and functions

use crate::{common::HashV1Json, v1::HashV1, version_bytes};
use serde::{Deserialize, Serialize};

/// Proof that the block was produced within the allotted slot time
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DeltaTransitionChainProof(pub HashV1, pub Vec<HashV1>);

/// Proof that the block was produced within the allotted slot time
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DeltaTransitionChainProofJson(
    pub HashV1Json<{ version_bytes::LEDGER_HASH }>,
    pub Vec<HashV1Json<{ version_bytes::LEDGER_HASH }>>,
);

impl From<DeltaTransitionChainProofJson> for DeltaTransitionChainProof {
    fn from(t: DeltaTransitionChainProofJson) -> Self {
        let (t0,): (HashV1,) = t.0.into();
        let t1: Vec<HashV1> =
            t.1.into_iter()
                .map(|i| {
                    let (i,): (HashV1,) = i.into();
                    i
                })
                .collect();
        Self(t0, t1)
    }
}

impl From<DeltaTransitionChainProof> for DeltaTransitionChainProofJson {
    fn from(t: DeltaTransitionChainProof) -> Self {
        Self(t.0.into(), t.1.into_iter().map(|i| i.into()).collect())
    }
}
