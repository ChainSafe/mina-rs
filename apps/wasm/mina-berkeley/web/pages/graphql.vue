<template>
  <NavBar />
  <div class="container px-4 prose">
    <h2>Mina Berkeley</h2>
    <p>wasm: {{ wasmStatus() }}</p>
    <p>API list: (each on a new line)</p>
    <textarea
      v-model="apiList"
      class="min-w-full"
    />
    <button
      v-if="wasmLoaded"
      class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
      @click="setApiList"
    >
      Set Api List
    </button>
    <p>Status: {{apiListStatus}}</p>
    <p>Tracking accounts list: (each on a new line)</p>
    <textarea
      v-model="accountList"
      class="min-w-full"
    />
    <button
      v-if="wasmLoaded"
      class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
      @click="setAccountsList"
    >
      Set Account List
    </button>
    <p>Status: {{accountListStatus}}</p>
  </div>
  <div class="container px-4 prose">
    <p>
      <button
        v-if="wasmLoaded"
        class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
        @click="pollLatestBlocksOnce"
      >
        Poll latest blocks
      </button>
    </p>
    <p>best chain state: {{ bestChainState }}</p>
    <p>sparse merkle ledger: {{ sparseMerkleLedger }}</p>
    <p>
      <button
        v-if="wasmLoaded"
        class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
        @click="exportLog"
      >
        Export log
      </button>
    </p>
    <p>
      NOTE: For now it requires disabling cors with
      <a
        href="https://chrome.google.com/webstore/detail/cors-unblock/lfhmikememgdcahcdlaciloancbhjino"
      >this extension</a>
    </p>
  </div>
</template>

<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import { initWasm, downloadFile, textToPath } from "~/utils";
import {
  initLogger,
  pollLatestBlocks,
  getBestChainState,
  runProcessor,
  get_log_text,
  getSparseMerkleLedger,
  setApiList,
  setTrackingAccounts
} from "~/pkg/wasm";

export default {
  components: {
    NavBar,
  },
  data() {
    return {
      wasmLoaded: false,
      apiList: ["http://localhost:3085/graphql"],
      accountList: ["B62qkCGtRNQ41gXbD8eXwUfY6w7bkPCUbHF18wzRFYxs9tErnchdDcr"],
      bestChainState: "",
      sparseMerkleLedger: "",
      apiListStatus: false,
      accountListStatus: false
    };
  },
  async created() {
    await this.loadWasm();
    runProcessor();
  },
  methods: {
    async loadWasm() {
      await initWasm();
      this.wasmLoaded = true;
      initLogger();
    },
    wasmStatus() {
      return this.wasmLoaded ? "loaded" : "loading";
    },
    async setApiList() {
      this.apiListStatus = await setApiList(this.apiList);
    },
    async setAccountsList() {
      this.accountListStatus = await setTrackingAccounts(this.accountList);
    },
    async pollLatestBlocksOnce() {
      this.bestChainState = `updating\n${this.bestChainState}`;
      this.sparseMerkleLedger = `updating\n${this.sparseMerkleLedger}`;
      await pollLatestBlocks();
      await this.refreshbestChainState();
      await this.refreshsparseMerkleLedger();
    },
    async refreshbestChainState() {
      const data = await getBestChainState();
      this.bestChainState = JSON.stringify(data, null, 2);
    },
    async refreshsparseMerkleLedger() {
      const data = await getSparseMerkleLedger();
      this.sparseMerkleLedger = JSON.stringify(data, null, 2);
    },
    exportLog() {
      const logText = get_log_text()
      console.log(logText)
      downloadFile(textToPath(logText))
    }
  },
};
</script>
<style lang="scss" scoped></style>
