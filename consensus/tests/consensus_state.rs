// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use mina_rs_base::{types::ConsensusState, JsonSerializationType};
    use test_fixtures::*;

    #[test]
    fn consensus_state_json_serde_tests() -> anyhow::Result<()> {
        for (_, json) in JSON_TEST_BLOCKS.iter() {
            let json = &json["protocol_state"]["body"]["consensus_state"];
            let json_string = serde_json::to_string_pretty(json)?;
            let cs = ConsensusState::try_from_json(&json_string)?;
            let json_string_from_cs = cs.try_into_json()?;
            let json_from_cs: serde_json::Value = serde_json::from_str(&json_string_from_cs)?;
            assert_eq!(json, &json_from_cs);
        }
        Ok(())
    }
}
