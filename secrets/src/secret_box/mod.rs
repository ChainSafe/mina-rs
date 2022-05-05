// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//!
//! This module contains utilties for loading keypair from wallet files
//! that are generated from mina-keypair-gen tool
//!
//! # Example
//! ```rust
//! use mina_secrets::secret_box::*;
//!
//! let wallet_json = include_str!("../../tests/data/test-wallet");
//! let sb: SecretBox = wallet_json.try_into().unwrap();
//! let wallet_json_exported: String = sb.try_into().unwrap();
//! ```
//!

mod constants;
mod types_impls;
mod utils;

mod errors;
pub use errors::*;

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

// Re-export Keypair
pub use proof_systems::mina_signer::Keypair;
