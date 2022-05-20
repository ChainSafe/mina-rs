<template>
  <NavBar />
  <div class="container px-4 prose">
    <h2>Mina Web Node Demo using graphql</h2>
    <p>wasm: {{ wasmStatus() }}</p>
    <p>peer list: (each on a new line)</p>
    <textarea
      v-model="nodeList"
      class="min-w-full"
    />
    <button
      v-if="wasmLoaded"
      class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
      @click="connect"
    >
      Connect
    </button>
  </div>
  <div class="container px-4 prose">
    <p>
      <span>Block height:</span><input
        v-model="heightToQuery"
        type="text"
      >
    </p>
    <p>
      <span>Block state hash:</span><input
        v-model="stateHashToQuery"
        type="text"
      >
    </p>
    <button
      v-if="wasmLoaded"
      class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white"
      @click="queryPreviousStateHash"
    >
      Query previous state hash
    </button>
    <p>
      NOTE: For now it requires disabling cors with
      <a
        href="https://chrome.google.com/webstore/detail/cors-unblock/lfhmikememgdcahcdlaciloancbhjino"
      >this extension</a>
    </p>
    <p>Previous state hash: {{ previousStateHash }}</p>
  </div>
</template>

<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import { initWasm } from "~/utils";
import { fetch_block_previous_state_hash } from "~/pkg/wasm";

export default {
  components: {
    NavBar,
  },
  data() {
    return {
      wasmLoaded: false,
      nodeList: "http://localhost:3085/graphql",
      heightToQuery: 25718,
      stateHashToQuery: "3NLQanLUpZLAbkciDnUs6bQGkgg48UqatpZxShHuLWSudG4M9iyn",
      previousStateHash: "",
    };
  },
  created() {
    this.loadWasm();
  },
  methods: {
    async loadWasm() {
      await initWasm();
      this.wasmLoaded = true;
    },
    wasmStatus() {
      return this.wasmLoaded ? "loaded" : "loading";
    },
    async connect() {
      alert("Not implemented yet");
    },
    async queryPreviousStateHash(height, stateHash) {
      try {
        this.previousStateHash = await fetch_block_previous_state_hash(
          this.heightToQuery,
          this.stateHashToQuery
        );
        console.log(this.previousStateHash);
      } catch (e) {
        this.previousStateHash = e.toString();
      }
    },
  },
};
</script>
<style lang="scss" scoped></style>
