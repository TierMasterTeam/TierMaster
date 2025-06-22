<script setup lang="ts">
import Footer from '../components/Footer.vue';

</script>

<template>
<div class="max-w-2xl mx-auto px-4 py-6 font-roboto">
  <h1 class="text-xl font-bold mb-4">{{ $t('termsOfUse.title') }}</h1>
  <div v-html="$t('termsOfUse.content')"></div>
</div>

</template>
