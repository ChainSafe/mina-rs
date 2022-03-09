import Vue from 'vue';
import App from './App.vue';
import 'regenerator-runtime/runtime'
import wasmUrl from "./../pkg/wasm_bg.wasm"
import { init } from "./../pkg/wasm"

async function initWasm() {
  await init(await fetch(wasmUrl as any))
}

new Vue({
  render: h => h(App),
}).$mount('#app');

initWasm()
