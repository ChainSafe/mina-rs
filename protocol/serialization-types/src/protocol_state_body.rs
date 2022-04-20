// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::v1::{BlockchainStateV1, ConsensusStateV1, HashV1, ProtocolConstantsV1};

/// Body of the protocol state
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolStateBody {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: HashV1,
    /// Ledger related state
    pub blockchain_state: BlockchainStateV1,
    /// Consensus related state
    pub consensus_state: ConsensusStateV1,
    /// Consensus constants
    pub constants: ProtocolConstantsV1,
}

/// Body of the protocol state (v1)
pub type ProtocolStateBodyV1 = Versioned<Versioned<ProtocolStateBody, 1>, 1>;

/// Body of the protocol state
/// that is convertible from / to the mina specific json representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolStateBodyJson {}

impl From<ProtocolStateBodyJson> for ProtocolStateBodyV1 {
    fn from(t: ProtocolStateBodyJson) -> Self {
        let t: ProtocolStateBody = t.into();
        Self::new(Versioned::new(t))
    }
}

impl From<ProtocolStateBodyV1> for ProtocolStateBodyJson {
    fn from(t: ProtocolStateBodyV1) -> Self {
        t.t.t.into()
    }
}

impl From<ProtocolStateBodyJson> for ProtocolStateBody {
    fn from(_t: ProtocolStateBodyJson) -> Self {
        unimplemented!()
    }
}

impl From<ProtocolStateBody> for ProtocolStateBodyJson {
    fn from(_t: ProtocolStateBody) -> Self {
        unimplemented!()
    }
}
