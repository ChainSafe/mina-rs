// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

pub(crate) const FILE_ID: &str = "MINA_SNARK_KEYS";

/// The header included in Mina serialized SNARK key files.
/// Describes parameters of the key and the type to expect 
/// to be included in the binary section of the file
#[derive(Debug, Deserialize)]
pub struct KeyFileHeader {
    pub header_version: u8,
    pub kind: Kind,
    pub constraint_constants: ConstraintConstants,
    pub commits: Commits,
    pub length: u64,
    pub commit_date: String,
    pub constraint_system_hash: String,
    pub identifying_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct Kind {
    pub r#type: KeyType,
    pub identifier: KeyIdentifier,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum KeyType {
    StepVerificationKey,
    WrapVerificationKey,
    StepProvingKey,
    WrapProvingKey,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KeyIdentifier {
    BlockchainSnarkStep,
    TransactionSnarkMerge,
    TransactionSnarkTransaction,
    BlockchainSnark,
    TransactionSnark,
}

#[derive(Debug, Deserialize)]
pub struct TransactionCapacity {
    pub two_to_the: u64,
}

#[derive(Debug, Deserialize)]
pub struct ConstraintConstants {
    pub sub_windows_per_window: u64,
    pub ledger_depth: u64,
    pub work_delay: u64,
    pub block_window_duration_ms: u64,
    pub transaction_capacity: TransactionCapacity,
    pub pending_coinbase_depth: u64,
    pub coinbase_amount: String,
    pub supercharged_coinbase_factor: u64,
    pub account_creation_fee: String,
    pub fork: Fork,
}

#[derive(Debug, Deserialize)]
pub struct Fork {
    pub previous_state_hash: Option<String>,
    pub previous_length: Option<u64>,
    pub previous_global_slot: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Commits {
    pub mina: String,
    pub marlin: String,
}
