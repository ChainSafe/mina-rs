// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use js_sys::Uint8Array;
use wasm_bindgen::JsError;

pub fn proto_msg_to_u8array<T: protobuf::Message>(msg: &T) -> anyhow::Result<Uint8Array> {
    let bytes = msg.write_to_bytes()?;
    let u8a = Uint8Array::new_with_length(bytes.len() as u32);
    u8a.copy_from(&bytes);
    Ok(u8a)
}

pub fn u8array_to_proto_msg<T: protobuf::Message>(u8a: Uint8Array) -> anyhow::Result<T> {
    let bytes = u8a.to_vec();
    Ok(T::parse_from_bytes(&bytes)?)
}

pub fn err_to_js_error<T: Display>(err: T) -> JsError {
    JsError::new(err.to_string().as_str())
}
