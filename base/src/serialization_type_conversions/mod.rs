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

impl_from_with_proxy!(StagedLedgerDiff, StagedLedgerDiffV1, StagedLedgerDiffJson);

impl_from_with_proxy!(
    crate::types::DeltaTransitionChainProof,
    mina_serialization_types::delta_transition_chain_proof::DeltaTransitionChainProof,
    DeltaTransitionChainProofJson
);
