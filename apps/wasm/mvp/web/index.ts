import { createApp } from "vue";
import App from "./App.vue";
import 'regenerator-runtime/runtime'
import wasmUrl from "./../pkg/wasm_bg.wasm"
import { init } from "./../pkg/wasm"

async function initWasm() {
  await init(await fetch(wasmUrl))
  const app = createApp(App);
  app.mount("#app");
}

initWasm()
