<template>
  <NavBar name="Index" />
  <div class="container px-4 prose">
    <h2>Mina Berkeley</h2>
    <p>
      <a href="https://github.com/ChainSafe/mina-rs/tree/main/apps/wasm/mina-berkeley">github</a>
    </p>
    <p v-html="readmeHTML" />
  </div>
</template>

<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import { marked } from "marked";
import readmeMD from "bundle-text:~/readme.md";

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
