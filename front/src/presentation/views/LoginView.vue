<script setup lang="ts">
import BaseInput from '../components/base/BaseInput.vue';
import Button from '../components/base/Button.vue';
import { ref } from 'vue';
import { Eye, EyeOff } from 'lucide-vue-next';
import { useAuthStore } from '../stores/authStore';
import { useUtilsStore } from '../stores/utilsStore';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();

const username = ref('');
const password = ref('');
const showPassword = ref(false);
const authStore = useAuthStore();
const showToast = useUtilsStore().showToast;

const useLogin = async () => {
  try {
    const success = await authStore.login({ 
      email: username.value, 
      password: password.value 
    });

    if (success) {
      showToast('Login successful', 'success');

      let redirect = route.query.redirect;
      if (typeof redirect === 'string') {
        router.push(redirect);
      } else {
        router.push({ name: 'home' });
      }
    } else {
      showToast('Login failed', 'error');
    }
  } catch (error) {
    showToast('Unable to connect to server', 'error');
  }
};

</script>

<template>
    <div class="bg-gradient-to-br from-dark-gray-custom from-[38.07%] to-dark-green-custom to-[131.95%] min-h-svh flex flex-col items-center justify-center">
        <form @submit.prevent="useLogin()">
            <div class="flex md:h-[132px] gap-8 flex-col md:flex-row">
                <img src="../../assets/logo.svg" width="167" height="132" alt="Logo" />
                <div class="flex flex-col items-center justify-between h-full gap-2">
                    <BaseInput type="text" placeholder="Username" class="block mt-1 h-12 w-80" v-model="username" />
                    <BaseInput :type="showPassword ? 'text' : 'password'" placeholder="Password" class="block h-12 mb-1 w-80" v-model="password">
                        <template #right>
                            <button type="button" @click="showPassword = !showPassword" tabindex="-1" aria-label="Afficher ou masquer le mot de passe">
                                <component :is="showPassword ? EyeOff : Eye" class="w-5 h-5 text-dark-gray-custom" />
                            </button>
                        </template>
                    </BaseInput>
                </div>
            </div>
            <div class="flex items-center gap-4 pt-8 pl-4">
                <Button type="submit" variant="primary" size="md" icon="login">
                    Login
                </Button>
                <router-link :to="{name: 'register'}" class="text-light-green-custom">
                    Create an account !
                </router-link>
            </div>
        </form>
    </div>
</template>