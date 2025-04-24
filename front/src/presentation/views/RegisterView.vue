<script setup lang="ts">
import BaseInput from '../components/base/BaseInput.vue';
import Button from '../components/base/Button.vue';
import { ref } from 'vue';
import { Eye, EyeOff } from 'lucide-vue-next';

const username = ref('');
const password = ref('');
const confirmPassword = ref('');
const showPassword = ref(false);
const showConfirmPassword = ref(false);

const useRegister = () => {
    // Implement your login logic here
    console.log('Login button clicked');
};
</script>

<template>
    <div class="bg-gradient-to-br from-dark-gray-custom from-[38.07%] to-dark-green-custom to-[131.95%] min-h-svh flex flex-col items-center justify-center">
        <form class="" @submit.prevent="useRegister()">
            <div class="flex md:h-[132px] gap-8 flex-col md:flex-row">
                <img src="../../assets/logo.svg" width="167" height="132" alt="Logo" />
                <div class="flex flex-col md:flex-row md:items-center justify-between h-full gap-6">
                    <div class="h-full flex flex-col justify-between gap-2">
                        <BaseInput type="text" placeholder="Username" class="block mt-1 h-12 w-80" v-model="username" />
                        <BaseInput :type="showPassword ? 'text' : 'password'" placeholder="Password" class="block h-12 mb-1 w-80" v-model="password">
                            <template #right>
                                <button type="button" @click="showPassword = !showPassword" tabindex="-1" aria-label="Afficher ou masquer le mot de passe">
                                    <component :is="showPassword ? EyeOff : Eye" class="w-5 h-5 text-dark-gray-custom" />
                                </button>
                            </template>
                        </BaseInput>
                    </div>
                    <div class="h-full flex flex-col justify-between gap-2">
                        <BaseInput type="text" placeholder="Username" class="block mt-1 h-12 w-80" />
                        <BaseInput :type="showConfirmPassword ? 'text' : 'password'" placeholder="Confirm Password" class="block h-12 mb-1 w-80" v-model="confirmPassword">
                            <template #right>
                                <button type="button" @click="showConfirmPassword = !showConfirmPassword" tabindex="-1" aria-label="Afficher ou masquer le mot de passe">
                                    <component :is="showConfirmPassword ? EyeOff : Eye" class="w-5 h-5 text-dark-gray-custom" />
                                </button>
                            </template>
                        </BaseInput>
                    </div>
                </div>
            </div>
            <div class="flex items-center gap-4 pt-8 pl-4">
                <Button type="submit" variant="primary" size="md" icon="login">
                    Signup
                </Button>
                <router-link :to="{name: 'login'}" class="text-light-green-custom">
                    Already have an account ?
                </router-link>
            </div>
        </form>
    </div>
</template>