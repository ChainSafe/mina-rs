// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use mina_crypto::hash::BaseHash;
use serde::{Deserialize, Serialize};

// TODO: confirm below types
pub type Length = u64;
pub type VrfOutput = [u8; 32];
pub type Amount = u64;

/// GlobalSlot contains (slot_number, slots_per_epoch)
pub type GlobalSlot = (u64, u64);

// TODO: public key type
pub type PublicKey = [u8; 32];

pub type Timestamp = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolState {
    pub previous_state_hash: BaseHash,
    pub body: ProtocolStateBody,
}

pub trait Header {
    fn get_height(&self) -> Length;
}

impl Header for ProtocolState {
    fn get_height(&self) -> Length {
        self.body.consensus_state.blockchain_length
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolStateBody {
    pub genesis_state_hash: BaseHash,
    pub blockchain_state: BlockchainState,
    pub consensus_state: ConsensusState,
    pub constants: Constants,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainState {
    staged_ledger_hash: StagedLedgerHash,
    snarked_ledger_hash: BaseHash,
    genesis_ledger_hash: BaseHash,
    snarked_next_available_token: u64,
    timestamp: Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusState {
    pub blockchain_length: Length,
    pub epoch_count: Length,
    pub min_window_density: Length,
    pub sub_window_densities: Vec<Length>,
    pub last_vrf_output: VrfOutput,
    pub total_currency: Amount,
    pub curr_global_slot: GlobalSlot,
    pub global_slot_since_genesis: u64,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
    pub has_ancestor_in_same_checkpoint_window: bool,
    pub block_stake_winner: PublicKey,
    pub block_creator: PublicKey,
    pub coinbase_receiver: PublicKey,
    pub supercharge_coinbase: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constants {
    pub k: Length,
    pub slots_per_epoch: Length,
    pub slots_per_sub_window: Length,
    pub delta: Length,
    pub genesis_state_timestamp: Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StagedLedgerHash {
    non_snark: NonSnark,
    pending_coinbase_hash: BaseHash,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpochData {
    ledger: Ledger,
    seed: BaseHash,
    start_checkpoint: BaseHash,
    lock_checkpoint: BaseHash,
    epoch_length: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    hash: BaseHash,
    total_currency: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonSnark {
    ledger_hash: BaseHash,
    aux_hash: BaseHash,
    pending_coinbase_aux: BaseHash, // TODO: was this removed? https://github.com/MinaProtocol/mina/blob/b137fbd750d9de1b5dfe009c12de134de0eb7200/src/lib/mina_base/staged_ledger_hash.mli
}
