import wasmUrl from "./pkg/wasm_bg.wasm"
import { init } from "./pkg/wasm"

export async function initWasm() {
    await init(await fetch(wasmUrl))
}
