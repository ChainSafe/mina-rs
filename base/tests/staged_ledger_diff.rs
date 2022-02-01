// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use mina_crypto::base58::Base58Encodable;
    use mina_rs_base::snapp_command::SnappCommand;
    use mina_rs_base::staged_ledger_diff::{
        CoinBase, CoinBaseBalanceData, CoinBaseFeeTransfer, FeeTransferBalanceData,
        TransactionStatus,
    };
    use mina_rs_base::types::*;
    use time::macros::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_CoinBaseFeeTransfer() {}
    #[test]
    #[wasm_bindgen_test]
    fn test_CoinBaseBalanceData() {}
    #[test]
    #[wasm_bindgen_test]
    fn test_FeeTransferBalanceData() {}
    #[test]
    #[wasm_bindgen_test]
    fn test_SnappCommand() {}
}
