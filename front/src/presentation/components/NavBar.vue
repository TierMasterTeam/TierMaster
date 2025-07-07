<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useAuthStore } from '../stores/authStore';

import { useI18n } from 'vue-i18n'
const route = useRoute()
const authStore = useAuthStore()
const isLoggedIn = authStore.isLoggedIn
const user = authStore.user

const { locale } = useI18n()
const switchLocale = (lang: string) => {
  locale.value = lang
  localStorage.setItem('locale', lang)
}

const useLogout = () => {
  authStore.logout()
  window.location.href = '/'
}
</script>

<template>
  <nav class="fixed bottom-0 md:bottom-auto md:top-0 left-0 right-0 h-[3.75rem] justify-between items-center flex px-5 border-t-1 md:border-t-0 md:border-b-1 border-b-white-custom bg-dark-gray-custom z-50">
    <div class="flex gap-9">
      <router-link class="hidden md:flex" to="/">
        <img src="../../assets/logo.svg" alt="tiermaster logo" width="45" height="36px">
      </router-link>
      <ul class="flex items-center text-light-green-custom gap-8 text-base">
        <li>
          <router-link
            to="/"
            class="nav-link"
            :class="{ 'nav-link-active': route.name === 'home' }"
          >{{ $t('nav.home') }}</router-link>
        </li>
        <li>
          <router-link
            :to="{name: 'myTemplates'}"
            class="nav-link"
            :class="{ 'nav-link-active': route.name === 'myTemplates' }"
          >{{ $t('nav.myTemplates') }}</router-link>
        </li>
        <li>
          <router-link
            :to="{name: 'myTierlists'}"
            class="nav-link"
            :class="{ 'nav-link-active': route.name === 'myTierlists' }"
          >{{ $t('nav.myTierlists') }}</router-link>
        </li>
      </ul>
    </div>
    <div class="flex items-center gap-4">
      <button @click="switchLocale(locale === 'fr' ? 'en' : 'fr')" class="lang-btn px-2 font-bold text-white">
        {{ locale === 'fr' ? 'fr' : 'en' }}
      </button>
      <template v-if="isLoggedIn">
        <router-link to="/profile">
          <img :src="'https://api.dicebear.com/9.x/bottts/svg?seed='+user?.username" alt="profile picture" class="rounded-full w-10 h-10 object-cover hover:ring-2 bg-gray-custom hover:ring-light-green-custom transition-all duration-300">
        </router-link>
      </template>
      <template v-else>
        <router-link to="/register" class="auth-link">{{ $t('nav.register') }}</router-link>
        <router-link to="/login" class="auth-link">{{ $t('nav.login') }}</router-link>
      </template>
    </div>

  </nav>
</template>

<style scoped>
.lang-btn {
  background: none;
  border: none;
  padding: 0 2px;
  cursor: pointer;
  outline: none;
  transition: transform 0.1s;
}
.lang-btn:active {
  transform: scale(0.95);
}
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

/* Add new styles for auth links */
.auth-link:hover,
.auth-link:focus {
  color: white;
  text-shadow: 0 0 8px white, 0 0 16px rgba(255, 255, 255, 0.7);
}

.auth-link:hover::after,
.auth-link:focus::after {
  background: white;
  opacity: 1;
  transform: scaleX(1);
  box-shadow: 0 0 16px white, 0 0 32px rgba(255, 255, 255, 0.5);
}
</style>
