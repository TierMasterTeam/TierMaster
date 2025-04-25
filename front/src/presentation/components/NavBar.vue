<script setup lang="ts">
import { useRoute } from 'vue-router'
const route = useRoute()
</script>

<template>
  <nav class="fixed bottom-0 md:bottom-auto md:top-0 left-0 right-0 h-[3.75rem] flex px-5 gap-9 border-t-1 md:border-t-0 md:border-b-1 border-b-white-custom bg-dark-gray-custom z-50">
    <router-link class="hidden md:flex" to="/">
      <img src="../../assets/logo.svg" alt="tiermaster logo" width="45" height="36px">
    </router-link>
    <ul class="flex items-center text-light-green-custom gap-8 text-base">
      <li>
        <router-link
          to="/"
          class="nav-link"
          :class="{ 'nav-link-active': route.name === 'home' }"
        >Home</router-link>
      </li>
      <li>
        <router-link
          :to="{name: 'createTemplate'}"
          class="nav-link"
          :class="{ 'nav-link-active': route.name === 'createTemplate' }"
        >create template</router-link>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.nav-link {
  position: relative;
  transition: color 0.3s;
  text-decoration: none;
}

.nav-link::after {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  bottom: -3px;
  height: 2px;
  border-radius: 2px;
  background: #31E7C3;
  opacity: 0;
  transform: scaleX(0.7);
  transition: opacity 0.3s, transform 0.3s, box-shadow 0.3s;
  box-shadow: 0 0 0px #31E7C3;
}

.nav-link:hover,
.nav-link:focus,
.nav-link-active {
  color: #31E7C3;
  text-shadow: 0 0 8px #31E7C3, 0 0 16px #1D7874;
}

.nav-link:hover::after,
.nav-link:focus::after,
.nav-link-active::after {
  opacity: 1;
  transform: scaleX(1);
  box-shadow: 0 0 16px #31E7C3, 0 0 32px #1D7874;
}
</style>
