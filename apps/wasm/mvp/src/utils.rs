// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub fn pb_to_u8a<T: protobuf::Message>(msg: &T) -> anyhow::Result<Uint8Array> {
    let bytes = msg.write_to_bytes()?;
    let u8a = Uint8Array::new_with_length(bytes.len() as u32);
    u8a.copy_from(&bytes);
    Ok(u8a)
}

pub fn u8a_to_pb<T: protobuf::Message>(u8a: Uint8Array) -> anyhow::Result<T> {
    let bytes = u8a.to_vec();
    Ok(T::parse_from_bytes(&bytes)?)
}

pub fn err_to_js_value(err: anyhow::Error) -> JsValue {
    JsValue::from_str(&format!("{}", err))
}
