// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const PREFIX_BYTE_LEN: usize = 20;
const PADDING_CHAR: u8 = b'*';

pub type HashPrefix = [u8; PREFIX_BYTE_LEN];

/// const function to create padded prefix strings with fixed length
/// that is being used in various of algorithms at compile time
const fn create(prefix: &[u8]) -> HashPrefix {
    let mut padded_prefix = [PADDING_CHAR; PREFIX_BYTE_LEN];
    let mut i = 0;
    while i < PREFIX_BYTE_LEN && i < prefix.len() {
        padded_prefix[i] = prefix[i];
        i += 1;
    }
    padded_prefix
}

pub const PROTOCOL_STATE: &HashPrefix = &create(b"CodaProtoState");

pub const PROTOCOL_STATE_BODY: &HashPrefix = &create(b"CodaProtoStateBody");

pub const ACCOUNT: &HashPrefix = &create(b"CodaAccount");

pub const SIDE_LOADED_VK: &HashPrefix = &create(b"CodaSideLoadedVk");

pub const SNAPP_ACCOUNT: &HashPrefix = &create(b"CodaSnappAccount");

pub const SNAPP_PAYLOAD: &HashPrefix = &create(b"CodaSnappPayload");

pub const SNAPP_BODY: &HashPrefix = &create(b"CodaSnappBody");

pub const MERGE_SNARK: &HashPrefix = &create(b"CodaMergeSnark");

pub const BASE_SNARK: &HashPrefix = &create(b"CodaBaseSnark");

pub const TRANSITION_SYSTEM_SNARK: &HashPrefix = &create(b"CodaTransitionSnark");

pub const SIGNATURE_TESTNET: &HashPrefix = &create(b"CodaSignature");

pub const SIGNATURE_MAINNET: &HashPrefix = &create(b"MinaSignatureMainnet");

pub const RECEIPT_CHAIN_USER_COMMAND: &HashPrefix = &create(b"CodaReceiptUC");

pub const RECEIPT_CHAIN_SNAPP: &HashPrefix = &create(b"CodaReceiptSnapp");

pub const EPOCH_SEED: &HashPrefix = &create(b"CodaEpochSeed");

pub const VRF_MESSAGE: &HashPrefix = &create(b"CodaVrfMessage");

pub const VRF_OUTPUT: &HashPrefix = &create(b"CodaVrfOutput");

pub const PENDING_COINBASES: &HashPrefix = &create(b"PendingCoinbases");

pub const COINBASE_STACK_DATA: &HashPrefix = &create(b"CoinbaseStackData");

pub const COINBASE_STACK_STATE_HASH: &HashPrefix = &create(b"CoinbaseStackStaHash");

pub const COINBASE_STACK: &HashPrefix = &create(b"CoinbaseStack");

pub const COINBASE: &HashPrefix = &create(b"Coinbase");

pub const CHECKPOINT_LIST: &HashPrefix = &create(b"CodaCheckpoints");

pub const BOWE_GABIZON_HASH: &HashPrefix = &create(b"CodaTockBGHash");

pub const SNAPP_PREDICATE: &HashPrefix = &create(b"CodaSnappPred");

pub const SNAPP_PREDICATE_ACCOUNT: &HashPrefix = &create(b"CodaSnappPredAcct");

pub const SNAPP_PREDICATE_PROTOCOL_STATE: &HashPrefix = &create(b"CodaSnappPredPS");

/// Builds a hash prefix for a node at the given depth in a Merkle tree
pub fn make_prefix_merkle_tree(i: usize) -> HashPrefix {
    let base = format!("CodaMklTree{:03}", i);
    create(base.as_bytes())
}

/// Builds a hash prefix for a node at the given depth in a coinbase Merkle tree
pub fn make_prefix_coinbase_merkle_tree(i: usize) -> HashPrefix {
    let base = format!("CodaCbMklTree{:03}", i);
    create(base.as_bytes())
}
