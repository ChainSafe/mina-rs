// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use serde::{Serialize, Deserialize};
use serde_versions_derive::version;

use mina_crypto::signature::PublicKey;

#[version(1)]
#[derive(Clone, Serialize, Deserialize)]
pub struct CoinbaseFeeTransfer {
	receiver_pk: PublicKey,
	fee: crate::numbers::currency::Fee,
}
