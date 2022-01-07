// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(test, feature = "browser"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use mina_crypto::base58::Base58Encodable;
    use mina_rs_base::staged_ledger_diff::{
        CoinBase, CoinBaseBalanceData, CoinBaseFeeTransfer, FeeTransferBalanceData, SnappCommand,
        TransactionStatus,
    };
    use time::macros::*;
    use wasm_bindgen_test::*;

    #[test]
    #[wasm_bindgen_test]
    fn test_CoinBaseFeeTransfer() {}

    fn test_CoinBaseBalanceData() {}
    fn test_FeeTransferBalanceData() {}
    fn test_CoinBase() {}
    fn test_TransactionStatus() {}
    fn test_SnappCommand() {}
}
