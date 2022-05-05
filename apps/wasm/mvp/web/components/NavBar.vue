<template>
  <nav
    class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 rounded dark:bg-gray-800"
  >
    <div class="container flex flex-wrap justify-between items-center mx-auto">
      <div class="hidden w-full md:block md:w-auto">
        <ul class="flex flex-row space-x-8 text-sm font-medium">
          <li
            v-for="page in pages"
            :key="page[1]"
          >
            <a
              v-if="!isActive(page)"
              :href="page[1]"
              class="block py-2 pr-4 pl-3 rounded text-blue-700"
              aria-current="page"
            >{{ page[0] }}</a>
            <span
              v-if="isActive(page)"
              class="block py-2 pr-4 pl-3 rounded text-stone-500"
              aria-current="page"
            >{{ page[0] }}</span>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
export default {
  props: ["name"],
  data() {
    return {
      pages: [
        ["Index", "index.html"],
        ["GraphQL", "graphql.html"],
        ["Libp2p", "libp2p.html"],
        ["About", "about.html"],
      ],
    };
  },
  methods: {
    isActive(page) {
      if (this.name) {
        return page[0].toLowerCase() == this.name.toLowerCase();
      }
      return window.location.pathname.indexOf(page[1]) >= 0;
    },
  },
};
</script>
<style lang="scss" scoped>
</style>
