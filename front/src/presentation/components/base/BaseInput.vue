<script setup lang="ts">
const model = defineModel<string>()
const props = defineProps<{ type?: string }>()
</script>

<template>
  <div class="relative flex items-center">
    <input
      v-bind="$attrs"
      :type="props.type || 'text'"
      v-model="model"
      class="rounded-lg px-4 py-2 bg-white-custom text-dark-gray-custom outline-none text-base w-full pr-10"
      id="base-input"
    />
    <label
      for="base-input"
      class="absolute right-3 cursor-pointer select-none flex items-center h-full"
    >
      <slot name="right" />
    </label>
  </div>
</template>
