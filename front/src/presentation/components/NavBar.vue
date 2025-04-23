<script setup lang="ts">
</script>

<template>
  <nav class="fixed top-0 left-0 right-0 h-[3.75rem] flex px-5 gap-9 border-b-1 border-b-white-custom">
    <router-link class="flex" to="/">
      <img src="../../assets/logo.svg" alt="tiermaster logo" width="45" height="36px">
    </router-link>
    <ul class="flex items-center text-light-green-custom gap-8 text-base">
      <li>
        <router-link to="/">Home</router-link>
      </li>
      <li>
        <router-link :to="{name: 'create-template'}">create template</router-link>
      </li>
    </ul>
  </nav>
</template>
