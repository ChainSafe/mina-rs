// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

import wasmUrl from "./pkg/wasm_bg.wasm"
import { init } from "./pkg/wasm"

export async function initWasm() {
    await init(await fetch(wasmUrl))
}
