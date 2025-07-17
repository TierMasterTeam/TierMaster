<template>
<footer class="hidden border-t-1 border-t-gray-custom md:flex justify-between px-5 h-[200px] py-9">
  <div class="left flex flex-col text-light-green-custom gap-6">
    <router-link class="block" :to="{name: 'privacyPolicy'}">
      {{ $t('footer.privacy') }}
    </router-link>
    <router-link class="block" :to="{name: 'termsOfUse'}">
      {{ $t('footer.terms') }}
    </router-link>
    <p class="text-white-custom">
      {{ $t('footer.noData') }}
    </p>
  </div>
  <div class="flex flex-col items-end gap-6 justify-end">
    <p>
      {{ $t('footer.contact') }}team.tiermaster@gmail.com
    </p>
    <p>
      {{ $t('footer.copyright') }}
    </p>
  </div>
</footer>
</template>
