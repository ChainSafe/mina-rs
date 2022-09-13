// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use crate::{
    blockchain_state::*,
    consensus_state::ConsensusState,
    from_graphql_json::FromGraphQLJson,
    global_slot::GlobalSlot,
    numbers::{BlockTime, Length},
    *,
};
use mina_crypto::hash::StateHash;
use mina_serialization_types::{json::*, v1::*};
use mina_serialization_types_macros::AutoFrom;
use proof_systems::{
    mina_hasher::{create_kimchi, create_legacy, Fp, Hashable, Hasher, ROInput},
    *,
};
use versioned::*;

/// Constants that define the consensus parameters
#[derive(Clone, Eq, PartialEq, Debug, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_constants::ProtocolConstants)]
pub struct ProtocolConstants {
    /// Point of finality (number of confirmations)
    pub k: Length,
    /// Number of slots per epoch
    pub slots_per_epoch: Length,
    /// No of slots in a sub-window = 7
    pub slots_per_sub_window: Length,
    /// Maximum permissable delay of packets (in slots after the current)
    pub delta: Length,
    /// Timestamp of genesis block in unixtime
    pub genesis_state_timestamp: BlockTime,
}

impl_from_with_proxy!(
    ProtocolConstants,
    ProtocolConstantsV1,
    ProtocolConstantsJson
);

impl Default for ProtocolConstants {
    fn default() -> Self {
        Self {
            k: 290.into(),
            slots_per_epoch: 7140.into(),
            slots_per_sub_window: 7.into(),
            delta: 0.into(),
            genesis_state_timestamp: 1655755201000.into(),
        }
    }
}

impl Hashable for ProtocolConstants {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.k)
            .append_hashable(&self.delta)
            .append_hashable(&self.slots_per_epoch)
            .append_hashable(&self.slots_per_sub_window)
            .append_hashable(&self.genesis_state_timestamp)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        None
    }
}

impl ToChunkedROInput for ProtocolConstants {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.k)
            .append_chunked(&self.delta)
            .append_chunked(&self.slots_per_epoch)
            .append_chunked(&self.slots_per_sub_window)
            .append_chunked(&self.genesis_state_timestamp)
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state::ProtocolState)]
/// This structure can be thought of like the block header. It contains the most essential information of a block. (legacy)
pub struct ProtocolStateLegacy {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBodyLegacy,
}

impl_from_with_proxy!(ProtocolStateLegacy, ProtocolStateV1, ProtocolStateJson);

impl Hashable for ProtocolStateLegacy {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        let mut hasher = create_legacy(());
        let body_hash = hasher.hash(&self.body);
        ROInput::new()
            .append_hashable(&self.previous_state_hash)
            .append_field(body_hash)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoState".into())
    }
}

impl ProtocolStateLegacy {
    /// Gets the current global slot the current epoch
    pub fn curr_global_slot(&self) -> &GlobalSlot {
        &self.body.consensus_state.curr_global_slot
    }

    /// Calculates the state hash field of current protocol state
    pub fn state_hash_fp(&self) -> Fp {
        let mut hasher = create_legacy(());
        hasher.hash(self)
    }

    /// Calculates the state hash of current protocol state
    pub fn state_hash(&self) -> StateHash {
        let f = self.state_hash_fp();
        (&f).into()
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, AutoFrom)]
#[auto_from(mina_serialization_types::protocol_state_body::ProtocolStateBody)]
/// Body of the protocol state (legacy)
pub struct ProtocolStateBodyLegacy {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: StateHash,
    /// Ledger related state
    pub blockchain_state: BlockchainStateLegacy,
    /// Consensus related state
    pub consensus_state: ConsensusState,
    /// Consensus constants
    pub constants: ProtocolConstants,
}

impl_from_with_proxy!(
    ProtocolStateBodyLegacy,
    ProtocolStateBodyV1,
    ProtocolStateBodyJson
);

impl Hashable for ProtocolStateBodyLegacy {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new()
            .append_hashable(&self.constants)
            .append_hashable(&self.genesis_state_hash)
            .append_hashable(&self.blockchain_state)
            .append_hashable(&self.consensus_state)
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoStateBody".into())
    }
}

/// Implementing types have some notion of height and can return it
pub trait Header {
    /// Get the height for the implementing type
    fn get_height(&self) -> Length;
    /// The minimum window density at the current epoch.
    fn min_window_density(&self) -> Length;
    /// A list of density values of the sub windows.
    fn sub_window_densities(&self) -> &Vec<Length>;
}

impl Header for ProtocolStateLegacy {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }

    fn sub_window_densities(&self) -> &Vec<Length> {
        &self.body.consensus_state.sub_window_densities
    }

    fn min_window_density(&self) -> Length {
        self.body.consensus_state.min_window_density
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
/// Body of the protocol state
pub struct ProtocolStateBody {
    /// Genesis protocol state hash (used for hardforks)
    pub genesis_state_hash: StateHash,
    /// Ledger related state
    pub blockchain_state: BlockchainState,
    /// Consensus related state
    pub consensus_state: ConsensusState,
    /// Consensus constants
    pub constants: ProtocolConstants,
}

impl FromGraphQLJson for ProtocolStateBody {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            // FIXME: Hard coded?
            genesis_state_hash: StateHash::from_str(
                "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
            )?,
            blockchain_state: BlockchainState::from_graphql_json(&json["blockchainState"])?,
            consensus_state: ConsensusState::from_graphql_json(&json["consensusState"])?,
            // FIXME: Hard coded?
            constants: Default::default(),
        })
    }
}

impl Hashable for ProtocolStateBody {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        self.roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoStateBody".into())
    }
}

impl ToChunkedROInput for ProtocolStateBody {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        ChunkedROInput::new()
            .append_chunked(&self.constants)
            .append_chunked(&self.genesis_state_hash)
            .append_chunked(&self.blockchain_state)
            .append_chunked(&self.consensus_state)
    }
}

impl From<ProtocolStateBodyLegacy> for ProtocolStateBody {
    fn from(t: ProtocolStateBodyLegacy) -> ProtocolStateBody {
        ProtocolStateBody {
            genesis_state_hash: t.genesis_state_hash,
            blockchain_state: t.blockchain_state.into(),
            consensus_state: t.consensus_state,
            constants: t.constants,
        }
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
/// This structure can be thought of like the block header. It contains the most essential information of a block.
pub struct ProtocolState {
    /// Commitment to previous block (hash of previous protocol state hash and body hash)
    pub previous_state_hash: StateHash,
    /// The body of the protocol state
    pub body: ProtocolStateBody,
}

impl FromGraphQLJson for ProtocolState {
    fn from_graphql_json(json: &serde_json::Value) -> anyhow::Result<Self> {
        Ok(Self {
            previous_state_hash: StateHash::from_str(
                json["previousStateHash"].as_str().unwrap_or_default(),
            )?,
            body: ProtocolStateBody::from_graphql_json(json)?,
        })
    }
}

impl Hashable for ProtocolState {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        self.roinput()
    }

    fn domain_string(_: Self::D) -> Option<String> {
        Some("CodaProtoState".into())
    }
}

impl ProtocolState {
    /// Gets the current global slot the current epoch
    pub fn curr_global_slot(&self) -> &GlobalSlot {
        &self.body.consensus_state.curr_global_slot
    }

    /// Calculates the state hash field of current protocol state
    pub fn state_hash_fp(&self) -> Fp {
        let mut hasher = create_kimchi(());
        hasher.hash(self)
    }

    /// Calculates the state hash of current protocol state
    pub fn state_hash(&self) -> StateHash {
        let f = self.state_hash_fp();
        (&f).into()
    }
}

impl ToChunkedROInput for ProtocolState {
    fn to_chunked_roinput(&self) -> ChunkedROInput {
        let mut hasher = create_kimchi(());
        let body_hash = hasher.hash(&self.body);
        ChunkedROInput::new()
            .append_chunked(&self.previous_state_hash)
            .append_field(body_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_state_from_graphql_json() -> anyhow::Result<()> {
        const JSON_STR: &str = r###"
        {
            "previousStateHash": "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
            "consensusState": {
              "blockCreator": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
              "blockHeight": "1",
              "blockStakeWinner": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
              "blockchainLength": "1",
              "coinbaseReceiever": "B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
              "epoch": "0",
              "epochCount": "0",
              "hasAncestorInSameCheckpointWindow": true,
              "lastVrfOutput": "48FthHHNE1y3YmoS4UvKaM6UyGd9nCJXTPVFQUGak7YtonTDUuEd",
              "minWindowDensity": "77",
              "slot": "0",
              "slotSinceGenesis": "0",
              "superchargedCoinbase": true,
              "totalCurrency": "1013238001000001000",
              "nextEpochData": {
                "epochLength": "2",
                "lockCheckpoint": "3NLUmnTBMCeExeWErijZ2GeLnjLtBgsDjN3qM8M8gcJDtk8k89xf",
                "seed": "2vc1zQHJx2xN72vaR4YDH31KwFSr5WHSEH2dzcfcq8jxBPcGiJJA",
                "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                "ledger": {
                  "hash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                  "totalCurrency": "1013238001000001000"
                }
              },
              "stakingEpochData": {
                "epochLength": "1",
                "lockCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                "seed": "2va9BGv9JrLTtrzZttiEMDYw1Zj6a6EHzXjmP9evHDTG3oEquURA",
                "startCheckpoint": "3NK2tkzqqK5spR2sZ7tujjqPksL45M3UUrcA4WhCkeiPtnugyE2x",
                "ledger": {
                  "hash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
                  "totalCurrency": "1013238001000001000"
                }
              }
            },
            "blockchainState": {
              "bodyReference": "36bda176656cc3be96c3d317db7b4ac06fdbc7f4eedcd6efdd20e28143d67421",
              "date": "1655755201000",
              "snarkedLedgerHash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
              "stagedLedgerAuxHash": "UDRUFHSvxUAtV8sh7gzMVPqpbd46roG1wzWR6dYvB6RunPihom",
              "stagedLedgerHash": "jwNYQU34Jb9FD6ZbKnWRALZqVDKbMrjZBKWFYZwAw8ZPMgv9Ld4",
              "stagedLedgerPendingCoinbaseHash": "2n27mUhCEctJbiZQdrk3kxYc7DVHvJVDErjXrjNs7jnP3HMLKtuN",
              "stagedLedgerPendingCoinbaseAux": "WAAeUjUnP9Q2JiabhJzJozcjiEmkZe8ob4cfFKSuq6pQSNmHh7",
              "stagedLedgerProofEmitted": false,
              "utcDate": "1655755201000"
            }
          }
        "###;
        let json = serde_json::from_str(JSON_STR)?;
        _ = ProtocolStateBody::from_graphql_json(&json)?;
        let ps = ProtocolState::from_graphql_json(&json)?;
        // Ensure the state hash we calculate is correct
        assert_eq!(
            ps.state_hash().to_string().as_str(),
            "3NKrvXDzp7gskxqWUmwDJTFeSGA6ohYMjd38uKwDgkg8RH89QcgH"
        );
        Ok(())
    }
}

impl Header for ProtocolState {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }

    fn sub_window_densities(&self) -> &Vec<Length> {
        &self.body.consensus_state.sub_window_densities
    }

    fn min_window_density(&self) -> Length {
        self.body.consensus_state.min_window_density
    }
}

impl From<ProtocolStateLegacy> for ProtocolState {
    fn from(t: ProtocolStateLegacy) -> ProtocolState {
        ProtocolState {
            previous_state_hash: t.previous_state_hash,
            body: t.body.into(),
        }
    }
}
