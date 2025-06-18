<template>
<footer class="hidden border-t-1 border-t-white-custom md:flex justify-between px-5 h-[200px] py-9">
  <div class="left flex flex-col text-light-green-custom gap-6">
    <router-link class="block" :to="{name: 'privacyPolicy'}">
      Privacy Policy
    </router-link>
    <router-link class="block" :to="{name: 'termsOfUse'}">
      Terms of Use
    </router-link>
    <p class="text-white-custom">
      No personal data collected • No use of cookies
    </p>
  </div>
  <div class="flex flex-col items-end gap-6 justify-end">
    <p>
      contact: lorem ipsum
    </p>
    <p>
      © 2025 TierMaster — All rights reserved
    </p>
  </div>
</footer>
</template>
