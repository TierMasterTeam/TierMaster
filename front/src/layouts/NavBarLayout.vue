<script setup lang="ts">
import NavBar from '@/presentation/components/NavBar.vue';
</script>
<template>
  <div class="min-h-screen flex flex-col pt-[3.75rem]">
    <NavBar />
    <main class="flex-1">
      <router-view />
    </main>
  </div>
</template>

