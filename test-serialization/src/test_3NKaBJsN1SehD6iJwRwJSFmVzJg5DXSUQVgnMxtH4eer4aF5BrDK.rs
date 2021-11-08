#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use pretty_assertions::assert_eq;
    use test_fixtures::*;

    // https://minaexplorer.com/block/3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK
    #[test]
    fn test_block() -> anyhow::Result<()> {
        let et = TEST_BLOCKS
            .get("3NKaBJsN1SehD6iJwRwJSFmVzJg5DXSUQVgnMxtH4eer4aF5BrDK.hex")
            .unwrap()
            .external_transition()?;

        let body = &et.protocol_state.body;

        let protocol_state = &body.blockchain_state;
        assert_eq!(protocol_state.timestamp.epoch_millis(), 1636092900000);
        assert_eq!(
            protocol_state.timestamp.datetime(),
            // FIXME: Why is min 18 in mina explorer?
            Utc.ymd(2021, 11, 5).and_hms_nano(6, 15, 0, 0)
        );

        let consensus_state = &body.consensus_state;
        assert_eq!(*consensus_state.blockchain_length, 77748);
        assert_eq!(*consensus_state.epoch_count, 15);
        assert_eq!(*consensus_state.curr_global_slot.slot_number, 111965);

        // TODO: total_currenty eq 867667132.840039233

        Ok(())
    }
}
