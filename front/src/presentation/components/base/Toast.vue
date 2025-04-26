<script setup lang="ts">
import { ref } from 'vue';
import { useUtilsStore } from '@/presentation/stores/utilsStore';

const toast = useUtilsStore().toast;
</script>

<template>
  <div
    class="fixed top-0 right-0 z-[9999] px-6 py-4 rounded-bl-lg font-bold transition-all duration-300 ease-in-out
      opacity-0 -translate-y-full
      "
    :class="[
      toast.isVisible ? 'opacity-100 translate-y-0' : '',
      toast.isSuccess ? 'bg-light-green-custom text-dark-gray-custom' : 'bg-red-custom text-white-custom',
    ]"
  >
    <p>{{ toast.content }}</p>
  </div>
</template>
