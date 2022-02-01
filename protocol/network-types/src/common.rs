// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Some basic versioned types used throughout

use versioned::Versioned;

pub type HashV1 = Versioned<[u8; 32], 1>;

pub type Hash2V1 = Versioned<HashV1, 1>;

pub type TokenIdV1 = Versioned<Versioned<Versioned<u64, 1>, 1>, 1>;

pub type BlockTimeV1 = Versioned<Versioned<u64, 1>, 1>;

pub type LengthV1 = Versioned<Versioned<u32, 1>, 1>;

pub type DeltaV1 = Versioned<Versioned<u32, 1>, 1>;

pub type GlobalSlotNumberV1 = Versioned<Versioned<u32, 1>, 1>;

pub type AmountV1 = Versioned<Versioned<u64, 1>, 1>;

// FIXME: 255 255 cannot be deserialized to u32, use i32 for now
// Note: Extended_Uint32 is not defined in bin_prot, but comes from mina
// Block path: t/staged_ledger_diff/t/diff/t/0/t/t/commands/0/t/data/t/t/t/t/payload/t/t/common/t/t/t/valid_until
/// u32 wrapped in 1 version byte
/// This will not be part of the public API once the deserialization refactor is complete
pub type ExtendedU32 = Versioned<Versioned<i32, 1>, 1>;

// u64 wrapped in 1 version byte
type ExtendedU64 = Versioned<u64, 1>;

/// u64 wrapped in 2 version bytes
/// This will not be part of the public API once the deserialization refactor is complete
pub type ExtendedU64_2 = Versioned<ExtendedU64, 1>;

/// u64 wrapped in 3 version bytes
/// This will not be part of the public API once the deserialization refactor is complete
pub type ExtendedU64_3 = Versioned<ExtendedU64_2, 1>;

pub type Hex64V1 = Versioned<i64, 1>;

pub type CharV1 = Versioned<u8, 1>;

pub type BigInt256 = [u8; 32];

pub type ByteVecV1 = Versioned<Vec<u8>, 1>;
