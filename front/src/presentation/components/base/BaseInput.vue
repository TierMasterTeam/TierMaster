<script setup lang="ts">
const model = defineModel<string>()
</script>

<template>
  <input
    v-bind="$attrs"
    v-model="model"
    class="rounded-lg px-4 py-2 bg-white-custom text-dark-gray-custom outline-none text-base"
  />
</template>
