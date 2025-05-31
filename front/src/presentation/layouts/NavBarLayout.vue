<script setup lang="ts">
import NavBar from '@/presentation/components/NavBar.vue';
import Footer from '@/presentation/components/Footer.vue';
</script>
<template>
  <div class="min-h-screen flex flex-col pb-[3.75rem] md:pb-0 md:pt-[3.75rem]">
    <NavBar />
    <main class="flex-1">
      <router-view />
    </main>
    <Footer />
  </div>
</template>

