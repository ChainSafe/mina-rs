<template>
  <NavBar name="Index" />
  <div class="container px-4 prose">
    <h2>Mina Web Node Demo</h2>
    <p>
      <a href="https://github.com/ChainSafe/mina-rs/tree/main/apps/wasm/mvp">github</a>
    </p>
    <p v-html="readmeHTML" />
  </div>
</template>

<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import { marked } from "marked";
import * as readmeMD from "~/readme.md";

export default {
  components: {
    NavBar,
  },
  data() {
    return {
      readmeHTML: "Failed to render readme.md",
    };
  },
  created() {
    try {
      this.readmeHTML = marked.parse(readmeMD);
    } catch {}
  },
};
</script>
<style lang="scss" scoped></style>
