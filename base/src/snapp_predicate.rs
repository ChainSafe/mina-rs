// Copyright 2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::epoch_data::EpochData;
use crate::numbers::Amount;
use serde::{Deserialize, Serialize};
use wire_type::WireType;

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug, WireType)]
#[serde(from = "<Self as WireType>::WireType")]
#[serde(into = "<Self as WireType>::WireType")]
#[wire_type(recurse = 2)]

/// https://github.com/MinaProtocol/mina/blob/aacfe04245d14b3331e89ed76a4b77bec902b290/src/lib/mina_base/snapp_predicate.ml#L529
///
pub struct ProtocolState {
    ///  TODO: Not sure if this should be frozen ledger hash or not
    ///
    pub currency: Amount,
    ///
    pub data: EpochData,
}
