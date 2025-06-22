<script setup lang="ts">
import Footer from '../components/Footer.vue';

</script>

<template>
  <div class="max-w-2xl mx-auto px-4 py-6">
  <h1 class="text-xl font-bold mb-4">{{ $t('privacyPolicy.title') }}</h1>
  <div v-html="$t('privacyPolicy.content')"></div>
</div>

</template>
