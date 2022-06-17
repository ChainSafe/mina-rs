// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

import wasmUrl from "raw:./pkg/wasm_bg.wasm"
import init from "./pkg/wasm"

export async function initWasm() {
    await init(await fetch(wasmUrl))
}

export function downloadFile(path, filename = 'log') {
    const anchor = document.createElement('a');
    anchor.href = path;
    anchor.download = filename;
    anchor.style.display = 'none';
    document.body.appendChild(anchor);
    anchor.click();
    document.body.removeChild(anchor);
}

export function textToPath(text, type = "text/plain") {
    const blob = new Blob([text], { type: type });
    return URL.createObjectURL(blob);
}
