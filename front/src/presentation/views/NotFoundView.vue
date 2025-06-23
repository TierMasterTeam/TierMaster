<template>
      <div class="bg-gradient-to-br from-dark-gray-custom from-[38.07%] to-dark-green-custom to-[131.95%] min-h-dvh flex flex-col items-center justify-center">
        <div>
          <h1 class="text-5xl md:text-[80px] font-jersey tracking-widest">
            404 Not Found
          </h1>
          <router-link :to="{name: 'home'}" class="text-light-green-custom md:text-lg text-left w-full">
            Back to application
          </router-link>
        </div>
      </div>

</template>
