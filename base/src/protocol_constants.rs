
use crate::{
    numbers::{BlockTime, Length},
};

use serde::{Serialize, Deserialize};

/// Protocol constants required for consensus and snarks. Consensus constants are generated using these

#[derive(Clone, Serialize, Deserialize)]
pub struct ProtocolConstants {
    /// Point of finality (number of confirmations)
    k: Length,
    /// Maximum permissable delay of packets (in slots after the current)
    delta: Length,
    slots_per_epoch: Length,
    slots_per_window: Length,
    /// Timestamp of genesis block in unixtime
    genesis_state_timestamp: BlockTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProtocolConstantsChecked(ProtocolConstants);

impl ProtocolConstants {
    pub fn check(self) -> ProtocolConstantsChecked {
        // TODO: implement checking logic here
        ProtocolConstantsChecked(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use toml::de::from_str;

    const CONF: &str = r#"
        delta = 0
        k = 290

        slots_per_epoch = 7140
        slots_per_window = 7

        genesis_state_timestamp = 1615939200000
    "#;

    #[test]
    fn can_deserialize_from_toml() {
        assert!(from_str::<ProtocolConstants>(CONF).is_ok());
    }
}