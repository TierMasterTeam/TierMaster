<script setup lang="ts">
import BaseInput from '../components/base/BaseInput.vue';
import { Search } from 'lucide-vue-next';

</script>
<template>
  <h1 class="font-jersey text-[80px] text-light-green-custom">TierMaster</h1>
  <BaseInput
    label="Search for a template"
    placeholder="Search..."
    class="w-fullblock"
    id="searchInput"
    >
    <template #right>
      <label for="base-input" class="cursor-pointer flex items-center">
        <Search class="h-5 w-5 text-light-gray-custom" />
      </label>
    </template>
  </BaseInput>
</template>
