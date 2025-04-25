<script setup lang="ts">
import { ref, onMounted } from 'vue'

const showBanner = ref(false)

onMounted(() => {
  // Check if the RGPD cookie already exists
  if (!document.cookie.split('; ').find(row => row.startsWith('tiermaster_rgpd_accepted='))) {
    showBanner.value = true
    setTimeout(() => {
      showBanner.value = false
      // Set the cookie for 1 year
      document.cookie = `tiermaster_rgpd_accepted=true; path=/; max-age=${60 * 60 * 24 * 365}`
    }, 4000)
  }
})
</script>

<template>
  <transition name="fade">
    <div v-if="showBanner" class="fixed bottom-4 left-1/2 -translate-x-1/2 z-50 bg-dark-gray-custom text-white-custom px-6 py-3 rounded-xl shadow-lg flex items-center gap-3 border border-light-green-custom animate-fade-in">
      <span class="font-semibold text-light-green-custom">No cookies or tracking are collected on this site.</span>
      <span class="hidden sm:inline text-xs text-gray-400">(RGPD banner - will disappear automatically)</span>
    </div>
  </transition>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.5s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>
