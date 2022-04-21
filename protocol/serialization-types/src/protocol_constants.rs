// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

//! Types related to the Mina protocol state

use serde::{Deserialize, Serialize};

use versioned::Versioned;

use crate::v1::{BlockTimeV1, LengthV1};

/// Constants that define the consensus parameters
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProtocolConstants {
    /// Point of finality (number of confirmations)
    pub k: LengthV1,
    /// Number of slots per epoch
    pub slots_per_epoch: LengthV1,
    /// No of slots in a sub-window = 7
    pub slots_per_sub_window: LengthV1,
    /// Maximum permissable delay of packets (in slots after the current)
    pub delta: LengthV1,
    /// Timestamp of genesis block in unixtime
    pub genesis_state_timestamp: BlockTimeV1,
}

/// Constants that define the consensus parameters (v1)
pub type ProtocolConstantsV1 = Versioned<Versioned<ProtocolConstants, 1>, 1>;
