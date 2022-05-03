// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::types::*;
use mina_serialization_types::{json::*, v1::*};
use versioned::*;

mod numbers;

impl_from_with_proxy!(
    ExternalTransition,
    ExternalTransitionV1,
    ExternalTransitionJson
);

impl_from_with_proxy!(
    ProtocolStateBody,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl_from_with_proxy!(ProtocolState, ProtocolStateV1, ProtocolStateJson);

impl From<CoinBase> for CoinBaseV1 {
    fn from(t: CoinBase) -> Self {
        use mina_serialization_types::staged_ledger_diff::CoinBase as CB;
        match t {
            CoinBase::Zero => Self::new(CB::Zero),
            CoinBase::One(maybe_fee) => Self::new(CB::One(maybe_fee.map(Into::into))),
            CoinBase::Two => Self::new(CB::Two),
        }
    }
}
impl From<CoinBaseV1> for CoinBase {
    fn from(t: CoinBaseV1) -> Self {
        use mina_serialization_types::staged_ledger_diff::CoinBase as CB;
        match t.t {
            CB::Zero => Self::Zero,
            CB::One(maybe_fee) => Self::One(maybe_fee.map(Into::into)),
            CB::Two => Self::Two,
            _ => unimplemented!(),
        }
    }
}

impl_from_with_proxy!(StagedLedgerDiff, StagedLedgerDiffV1, StagedLedgerDiffJson);

use mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof as DeltaTransitionChainProofV1;

impl From<crate::types::DeltaTransitionChainProof> for DeltaTransitionChainProofV1 {
    fn from(t: crate::types::DeltaTransitionChainProof) -> Self {
        Self(t.0.into(), t.1.into_iter().map(Into::into).collect())
    }
}
impl From<DeltaTransitionChainProofV1> for crate::types::DeltaTransitionChainProof {
    fn from(t: DeltaTransitionChainProofV1) -> Self {
        Self(t.0.into(), t.1.into_iter().map(Into::into).collect())
    }
}
impl_from_with_proxy!(
    crate::types::DeltaTransitionChainProof,
    DeltaTransitionChainProofV1,
    DeltaTransitionChainProofJson
);
