///
/// All human readable values (e.g base58 encoded hashes and addresses)
/// implement the Base58Checked encoding https://en.bitcoin.it/wiki/Base58Check_encoding
///
/// This adds a unique prefix byte to each type of encoding so they cannot be confused
/// (e.g. a hash cannot be used as an address). It also adds checksum bytes to the end.
///

pub const COINBASE: u8 = 0x01;

pub const SECRET_BOX_BYTESWR: u8 = 0x02;

pub const FEE_TRANSFER_SINGLE: u8 = 0x03;

pub const FRONTIER_HASH: u8 = 0x04;

pub const LEDGER_HASH: u8 = 0x05;

pub const LITE_PRECOMPUTED: u8 = 0x06;

// ---------------------------------

pub const PROOF: u8 = 0x0A;

pub const RANDOM_ORACLE_BASE: u8 = 0x0B;

pub const RECEIPT_CHAIN_HASH: u8 = 0x0C;

pub const EPOCH_SEED: u8 = 0x0D;

pub const STAGED_LEDGER_HASH_AUX_HASH: u8 = 0x0E;

pub const STAGED_LEDGER_HASH_PENDING_COINBASE_AUX: u8 = 0x0F;

pub const STATE_HASH: u8 = 0x10;

pub const STATE_BODY_HASH: u8 = 0x11;

pub const TRANSACTION_HASH: u8 = 0x12;

pub const USER_COMMAND: u8 = 0x13;

pub const USER_COMMAND_MEMO: u8 = 0x14;

pub const VRF_TRUNCATED_OUTPUT: u8 = 0x15;

pub const WEB_PIPE: u8 = 0x16;

pub const COINBASE_STACK_DATA: u8 = 0x17;

pub const COINBASE_STACK_HASH: u8 = 0x18;

pub const PENDING_COINBASE_HASH_BUILDER: u8 = 0x19;

pub const SNAPP_COMMAND: u8 = 0x1A;

/// The following version bytes are non-sequential; existing
/// user key infrastructure depends on them. don't change them!

pub const PRIVATE_KEY: u8 = 0x5A;

pub const NON_ZERO_CURVE_POINT: u8 = 0xCE;

pub const NON_ZERO_CURVE_POINT_COMPRESSED: u8 = 0xCB;

pub const SIGNATURE: u8 = 0x9A;
