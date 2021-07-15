use mina_crypto::hash::BaseHash;
use serde::{Serialize, Deserialize};

// TODO: confirm below types
pub type Length = u64;
pub type VrfOutput = [u8; 32];
pub type Amount = u64;
pub type GlobalSlot = (u64, u64);

// TODO: public key type
pub type PublicKey = [u8; 32];

pub type Timestamp = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolState {
    previous_state_hash: BaseHash,
    body: ProtocolStateBody,
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
    genesis_state_hash: BaseHash,
    blockchain_state: BlockchainState,
    consensus_state: ConsensusState,
    constants: Constants,
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
    blockchain_length: Length,
    epoch_count: Length,
    min_window_density: Length,
    sub_window_densities: Vec<Length>,
    last_vrf_output: VrfOutput,
    total_currency: Amount,
    curr_global_slot: GlobalSlot,
    global_slot_since_genesis: u64,
    staking_epoch_data: EpochData,
    next_epoch_data: EpochData,
    has_ancestor_in_same_checkpoint_window: bool,
    block_stake_winner: PublicKey,
    block_creator: PublicKey,
    coinbase_receiver: PublicKey,
    supercharge_coinbase: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constants {
    k: Length,
    slots_per_epoch: Length,
    slots_per_sub_window: Length,
    delta: Length,
    genesis_state_timestamp: Timestamp,
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
