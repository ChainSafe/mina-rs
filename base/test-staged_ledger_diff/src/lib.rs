#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use mina_crypto::base58::Base58Encodable;
    use mina_rs_base::staged_ledger_diff::{
        CoinBase, InternalCommandBalanceData, SignedCommandMemo, SignedCommandPayloadBody,
        TransactionStatus, UserCommand,
    };
    use time::macros::*;
    use wasm_bindgen_test::*;

    // #[test]
    // fn
}
