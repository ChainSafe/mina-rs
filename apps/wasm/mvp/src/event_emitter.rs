// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::*;

// FIXME: Remove unsafe code
static mut EVENT_EMITTER: Option<EventEmitter> = None;

// unsafe justifacation: WASM runs in single thread
pub fn set_event_emitter(e: EventEmitter) {
    unsafe { EVENT_EMITTER = Some(e) };
}

// unsafe justifacation: WASM runs in single thread
pub fn get_event_emitter<'a>() -> Option<&'a EventEmitter> {
    unsafe { EVENT_EMITTER.as_ref() }
}
