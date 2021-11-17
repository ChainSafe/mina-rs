// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

/// Mina SNARK keys are encoded in a dedicated, self documenting, file format
///
/// Snark key files have a header and body section
/// The header begins with a string describing the type of snark key (currently always "MINA_SNARK_KEYS")
/// The second line is json formatted and describes the key parameters
/// The remainder is the bin_prot encoded key data of the given type
///
/// For the full specification see https://github.com/MinaProtocol/mina/blob/f88edb440e321114e26f7691e599adab30ce16cd/docs/specs/types_and_structures/serialized_key.md
use serde::Deserialize;

pub(crate) const FILE_ID: &str = "MINA_SNARK_KEYS";

#[derive(Debug, Deserialize)]
pub struct KeyFileHeader {
    header_version: u8,
    kind: Kind,
    constraint_constants: ConstraintConstants,
    commits: Commits,
    length: u64,
    commit_date: String,
    constraint_system_hash: String,
    identifying_hash: String,
}

#[derive(Debug, Deserialize)]
struct Kind {
    r#type: KeyType,
    identifier: KeyIdentifier,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum KeyType {
    StepVerificationKey,
    WrapVerificationKey,
    StepProvingKey,
    WrapProvingKey,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum KeyIdentifier {
    BlockchainSnarkStep,
    TransactionSnarkMerge,
    TransactionSnarkTransaction,
    BlockchainSnark,
    TransactionSnark,
}

#[derive(Debug, Deserialize)]
struct TransactionCapacity {
    two_to_the: u64,
}

#[derive(Debug, Deserialize)]
struct ConstraintConstants {
    sub_windows_per_window: u64,
    ledger_depth: u64,
    work_delay: u64,
    block_window_duration_ms: u64,
    transaction_capacity: TransactionCapacity,
    pending_coinbase_depth: u64,
    coinbase_amount: String,
    supercharged_coinbase_factor: u64,
    account_creation_fee: String,
    fork: Fork,
}

#[derive(Debug, Deserialize)]
struct Fork {
    previous_state_hash: Option<String>,
    previous_length: Option<u64>,
    previous_global_slot: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Commits {
    mina: String,
    marlin: String,
}