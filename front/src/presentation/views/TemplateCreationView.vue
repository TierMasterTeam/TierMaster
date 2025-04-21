<script setup lang="ts">
import Button from '../components/base/Button.vue';
import BaseInput from '../components/base/BaseInput.vue';
import { ref } from 'vue';
const templateName = ref<string>('');
const templateCategories = ref<string[]>([]);
const currentCategory = ref<string>('');

const addCategory = () => {
  if (currentCategory.value.trim() !== '') {
    templateCategories.value.push(currentCategory.value.trim());
    currentCategory.value = '';
  }
};


</script>

<template>
  <main class="m-16">
    <label for="templateName">
      <h1 class="text-[40px] font-jersey">Tierlist Title</h1>
    </label>
    <BaseInput
      id="templateName"
      type="text"
      placeholder="My Tierlist title ..."
      class="mb-4 w-1/3 max-w-2xl"
      v-model="templateName"
    />
    <label for="category">
      <h2 class="text-2xl font-jersey">Add #categories</h2>
    </label>
    <BaseInput
      id="category"
      type="text"
      placeholder="category ..."
      class="mb-4 max-w-sm"
      v-model="currentCategory"
    />
    <Button
      variant="primary"
      type="button"
      size="md"
      icon="plus"
      class="ml-4 inline-flex"
      @click="addCategory"
    >
      Add Category
  </Button>
    <ul class="flex gap-2">
      <li v-for="(category, index) in templateCategories" :key="index">
        <span class="text-xl font-jersey bg-gray-custom rounded-full px-2 py-2">{{ category }}</span>
      </li>
    </ul>
    <Button variant="primary" type="button" size="md" icon="plus" class="mt-4">
      Create Template
    </Button>
  </main>
</template>
