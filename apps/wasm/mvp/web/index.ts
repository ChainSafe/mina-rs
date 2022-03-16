import { createApp } from "vue";
import App from "./App.vue";
import 'regenerator-runtime/runtime'
import { initWasm } from "./../utils"

async function buildApp() {
  await initWasm()
  const app = createApp(App);
  app.mount("#app");
}

buildApp()
