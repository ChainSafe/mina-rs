// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const PREFIX_BYTE_LEN: usize = 20;
const PADDING_CHAR: u8 = b'*';

pub type HashPrefix = &'static [u8; PREFIX_BYTE_LEN];

const fn create(s: &[u8]) -> HashPrefix {
    let len = s.len();
    let mut o = [PADDING_CHAR; PREFIX_BYTE_LEN];
    // no loops allowed in const fn. We can unroll it for 20 elements but its not pretty
    if 0 < len {
        o[0] = s[0]
    }
    if 1 < len {
        o[1] = s[1]
    }
    if 2 < len {
        o[2] = s[2]
    }
    if 3 < len {
        o[3] = s[3]
    }
    if 4 < len {
        o[4] = s[4]
    }
    if 5 < len {
        o[5] = s[5]
    }
    if 6 < len {
        o[6] = s[6]
    }
    if 7 < len {
        o[7] = s[7]
    }
    if 8 < len {
        o[8] = s[8]
    }
    if 9 < len {
        o[9] = s[9]
    }
    if 10 < len {
        o[10] = s[10]
    }
    if 11 < len {
        o[11] = s[11]
    }
    if 12 < len {
        o[12] = s[12]
    }
    if 13 < len {
        o[13] = s[13]
    }
    if 14 < len {
        o[14] = s[14]
    }
    if 15 < len {
        o[15] = s[15]
    }
    if 16 < len {
        o[16] = s[16]
    }
    if 17 < len {
        o[17] = s[17]
    }
    if 18 < len {
        o[18] = s[18]
    }
    if 19 < len {
        o[19] = s[19]
    }
    &o
}

pub const PROTOCOL_STATE: HashPrefix = create(b"CodaProtoState");

pub const PROTOCOL_STATE_BODY: HashPrefix = create(b"CodaProtoStateBody");

pub const ACCOUNT: HashPrefix = create(b"CodaAccount");

pub const SIDE_LOADED_VK: HashPrefix = create(b"CodaSideLoadedVk");

pub const SNAPP_ACCOUNT: HashPrefix = create(b"CodaSnappAccount");

pub const SNAPP_PAYLOAD: HashPrefix = create(b"CodaSnappPayload");

pub const SNAPP_BODY: HashPrefix = create(b"CodaSnappBody");

pub const MERGE_SNARK: HashPrefix = create(b"CodaMergeSnark");

pub const BASE_SNARK: HashPrefix = create(b"CodaBaseSnark");

pub const TRANSITION_SYSTEM_SNARK: HashPrefix = create(b"CodaTransitionSnark");

pub const SIGNATURE_TESTNET: HashPrefix = create(b"CodaSignature");

pub const SIGNATURE_MAINNET: HashPrefix = create(b"MinaSignatureMainnet");

pub const RECEIPT_CHAIN_USER_COMMAND: HashPrefix = create(b"CodaReceiptUC");

pub const RECEIPT_CHAIN_SNAPP: HashPrefix = create(b"CodaReceiptSnapp");

pub const EPOCH_SEED: HashPrefix = create(b"CodaEpochSeed");

pub const VRF_MESSAGE: HashPrefix = create(b"CodaVrfMessage");

pub const VRF_OUTPUT: HashPrefix = create(b"CodaVrfOutput");

pub const PENDING_COINBASES: HashPrefix = create(b"PendingCoinbases");

pub const COINBASE_STACK_DATA: HashPrefix = create(b"CoinbaseStackData");

pub const COINBASE_STACK_STATE_HASH: HashPrefix = create(b"CoinbaseStackStaHash");

pub const COINBASE_STACK: HashPrefix = create(b"CoinbaseStack");

pub const COINBASE: HashPrefix = create(b"Coinbase");

pub const CHECKPOINT_LIST: HashPrefix = create(b"CodaCheckpoints");

pub const BOWE_GABIZON_HASH: HashPrefix = create(b"CodaTockBGHash");

pub const SNAPP_PREDICATE: HashPrefix = create(b"CodaSnappPred");

pub const SNAPP_PREDICATE_ACCOUNT: HashPrefix = create(b"CodaSnappPredAcct");

pub const SNAPP_PREDICATE_PROTOCOL_STATE: HashPrefix = create(b"CodaSnappPredPS");

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

#[cfg(test)]
mod tests {
    use crate::hash_prefixes::{make_prefix_coinbase_merkle_tree, PROTOCOL_STATE};

    #[test]
    fn create_works_as_expected() {
        assert_eq!(PROTOCOL_STATE.len(), 20);
        assert_eq!(PROTOCOL_STATE, b"CodaProtoState******");
    }

    #[test]
    fn make_merkle_tree_hash() {
        let prefix_at_3 = make_prefix_coinbase_merkle_tree(3);
        assert_eq!(prefix_at_3.len(), 20);
        assert_eq!(prefix_at_3, b"CodaCbMklTree003****");
    }
}
