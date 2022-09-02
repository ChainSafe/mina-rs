// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const PAYMENT: &'static str = r#"
export interface Payment {
    to: PublicKey;
    from: PublicKey;
    fee: UInt64;
    amount: UInt64;
    nonce?: UInt32;
    memo?: string;
    validUntil?: UInt32;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Payment")]
    pub type Payment;

    #[wasm_bindgen(method, getter)]
    pub fn to(this: &Payment) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn from(this: &Payment) -> String;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn fee(this: &Payment) -> JsValue;

    // u64 can be either f64 or bigint in js,
    // use JsValue to handle the conversion
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Payment) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &Payment) -> Option<u32>;

    #[wasm_bindgen(method, getter)]
    pub fn memo(this: &Payment) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = validUntil)]
    pub fn valid_until(this: &Payment) -> JsValue;
}

impl Payment {
    pub fn fee_u64(&self) -> Result<u64, JsError> {
        js_to_string(&self.fee()).parse().map_err(map_js_err)
    }

    pub fn amount_u64(&self) -> Result<u64, JsError> {
        js_to_string(&self.amount()).parse().map_err(map_js_err)
    }

    pub fn valid_until_u32(&self) -> Result<Option<u32>, JsError> {
        let valid_until = self.valid_until();
        if valid_until.is_null() || valid_until.is_undefined() {
            Ok(None)
        } else {
            let s = js_to_string(&valid_until);
            if s.is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.parse().map_err(map_js_err)?))
            }
        }
    }
}

#[wasm_bindgen(js_name = accountInfo)]
pub async fn account_info(pub_key: String) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "query MyQuery {{  
            account(publicKey: \"{}\") {{ balance {{ total }} inferredNonce locked  }}
        }}",
        pub_key
    );
    let res = client
        .post("http://localhost:3085/graphql")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}

#[wasm_bindgen(js_name = lockAccount)]
pub async fn lock_account(pub_key: String) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            lockAccount(input: {{publicKey: \"{}\"}}) {{    account {{      locked    }}  }}
        }}",
        pub_key
    );
    let res = client
        .post("http://localhost:3085/graphql")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}

#[wasm_bindgen(js_name = unlockAccount)]
pub async fn unlock_account(pub_key: String, password: String) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            unlockAccount(input: {{publicKey: \"{}\", password: \"{}\"}}) {{    account {{      locked    }}  }}
        }}", pub_key, password
    );
    let res = client
        .post("http://localhost:3085/graphql")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}

#[wasm_bindgen(js_name = sendPayment)]
pub async fn send_payment(payment: Payment) -> Result<String, JsError> {
    let client = reqwest::Client::new();
    let query: String = format!(
        "mutation MyMutation {{  
            sendPayment(input: {{fee: \"{}\", amount: \"{}\", to: \"{}\", from: \"{}\"}}) {{    payment {{      id    }}  }}
        }}", payment.fee_u64()?, payment.amount_u64()?, payment.to(), payment.from()
    );
    let res = client
        .post("http://localhost:3085/graphql")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await
        .unwrap();
    let resp_text = res.text().await.unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp_text).unwrap();
    Ok(resp.to_string())
}
