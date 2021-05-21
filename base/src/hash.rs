//!
//! Hash and Hasher types reused throughout
//!

pub use sha2::Sha256 as DefaultHasher;

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct Hash(u64);

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct StateHash(Hash);

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct LedgerHash(Hash);

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct EpochSeed(Hash);
