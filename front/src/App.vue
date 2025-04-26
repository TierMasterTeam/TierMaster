<script setup lang="ts">
import { RouterView } from 'vue-router'
import RgpdBanner from './presentation/components/RgpdBanner.vue'
import Toast from './presentation/components/base/Toast.vue';
</script>

<template>
  <Toast />
  <RgpdBanner />
  <RouterView />
</template>
