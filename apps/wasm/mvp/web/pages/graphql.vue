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
</template>

<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import { initWasm } from "~/utils";

export default {
  components: {
    NavBar,
  },
  data() {
    return {
      wasmLoaded: false,
      nodeList: "http://localhost:3085/graphql",
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
  },
};
</script>
<style lang="scss" scoped></style>
