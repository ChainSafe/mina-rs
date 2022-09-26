// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::payment::Payment;
use crate::proof::DefaultMerkleProof;
use crate::protocol_state::ProtocolState;
use crate::{logger::JsExportableLogger, *};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn run_processor() {
    frontier::PROCESSOR_BERKELEY.run().await;
}

#[wasm_bindgen]
pub async fn set_api_list(api_urls: Array) -> Result<(), JsError> {
    let mut backend = frontier::PROCESSOR_BERKELEY.nonconsensus_ops_mut().await;
    let api_vec = api_urls.to_vec();
    let api_list = api_vec.iter().map(|url| url.as_string().unwrap());
    backend.set_api_list(api_list);
    Ok(())
}

#[wasm_bindgen]
pub async fn set_tracking_accounts(accounts: Array) -> Result<(), JsError> {
    let mut backend = frontier::PROCESSOR_BERKELEY.nonconsensus_ops_mut().await;
    let acc_vec = accounts.to_vec();
    let account_list = acc_vec.iter().map(|acc| acc.as_string().unwrap());
    backend.set_tracking_accounts(account_list);
    Ok(())
}

#[wasm_bindgen]
pub async fn poll_latest_blocks_once() -> Result<(), JsError> {
    let backend = frontier::PROCESSOR_BERKELEY.nonconsensus_ops().await;
    backend.poll_latest_once().await.map_err(err_to_js_error)
}

#[wasm_bindgen]
pub async fn get_best_chain_state_hash() -> Option<String> {
    let frontier = frontier::PROCESSOR_BERKELEY.transition_frontier().await;
    let chain = frontier.get_best_chain();
    chain.state_hash().map(|state_hash| state_hash.to_string())
}

#[wasm_bindgen]
pub async fn get_best_chain_state() -> Option<ProtocolState> {
    let frontier = frontier::PROCESSOR_BERKELEY.transition_frontier().await;
    let chain = frontier.get_best_chain();
    chain.top().map(|protocol_state| {
        let ps: ProtocolState = protocol_state.clone().into();
        ps
    })
}

#[wasm_bindgen]
pub async fn get_sparse_merkle_ledger() -> Array {
    let frontier = frontier::PROCESSOR_BERKELEY.transition_frontier().await;
    let sparse_merkle_ledger = frontier.get_sparse_merkle_ledger();
    let data = sparse_merkle_ledger.iter().map(|ledger| {
        let js: DefaultMerkleProof = ledger.into();
        js
    });
    data.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
pub fn init_logger() -> Result<(), JsError> {
    static JS_LOGGER: JsExportableLogger = JsExportableLogger::new(log::Level::Debug);
    log::set_max_level(JS_LOGGER.max_level().to_level_filter());
    log::set_logger(&JS_LOGGER).map_err(err_to_js_error)
}

#[wasm_bindgen(js_name = lockAccount)]
pub async fn lock_account(endpoint: String, pub_key: String) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            lockAccount(input: {{publicKey: \"{}\"}}) {{    account {{      locked    }}  }}
        }}",
        pub_key
    );
    let res = client
        .post(endpoint)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}

#[wasm_bindgen(js_name = unlockAccount)]
pub async fn unlock_account(
    endpoint: String,
    pub_key: String,
    password: String,
) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            unlockAccount(input: {{publicKey: \"{}\", password: \"{}\"}}) {{    account {{      locked    }}  }}
        }}", pub_key, password
    );
    let res = client
        .post(endpoint)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}

#[wasm_bindgen(js_name = sendPayment)]
pub async fn send_payment(endpoint: String, payment: Payment) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            sendPayment(input: {{fee: \"{}\", amount: \"{}\", to: \"{}\", from: \"{}\"}}) {{    payment {{      id    }}  }}
        }}", payment.fee_u64()?, payment.amount_u64()?, payment.to(), payment.from()
    );
    let res = client
        .post(endpoint)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}
