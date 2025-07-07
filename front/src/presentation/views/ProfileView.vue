<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAuthStore } from '../stores/authStore'
import Button from '../components/base/Button.vue'
import { User } from 'lucide-vue-next'

const authStore = useAuthStore()
const user = authStore.user

const isEditing = ref(false)
const editForm = ref({
  username: user?.username || ''
})

onMounted(() => {
  if (user) {
    editForm.value = {
      username: user.username
    }
  }
})

const toggleEdit = () => {
  isEditing.value = !isEditing.value
  if (!isEditing.value) {
    // Reset form if canceling edit
    editForm.value = {
      username: user?.username || ''
    }
  }
}

const saveProfile = async () => {
  try {
    // TODO: Implement API call to update user profile
    // await authStore.updateProfile(editForm.value)
    isEditing.value = false
  } catch (error) {
    console.error('Error saving profile:', error)
  }
}

const logout = () => {
  authStore.logout()
  window.location.href = '/'
}
</script>

<template>
  <div class="flex flex-col p-16 w-full max-w-2xl mx-auto">
    <div class="flex items-center gap-4 mb-8">
      <img :src="'https://api.dicebear.com/9.x/bottts/svg?seed='+user?.username" class="w-20 h-20 bg-gray-custom rounded-full flex items-center justify-center"/>
      <div>
        <h1 class="text-[40px] font-jersey text-white-custom">
            {{ $t('profile.title') }}
        </h1>
        <p class="text-gray-300">{{ $t('profile.subtitle') }}</p>
      </div>
    </div>

    <div class="bg-dark-gray-custom border border-white-custom rounded-lg p-6">
      <div class="space-y-4">
        <div class="flex flex-col md:flex-row items-start md:items-center justify-between mb-6">
          <h2 class="text-xl md:pb-2 font-bold text-white-custom">{{ $t('profile.information') }}</h2>
          <Button
            size="sm"
            :variant="isEditing ? 'secondary' : 'primary'"
            @click="toggleEdit"
          >
            {{ isEditing ? $t('profile.cancel') : $t('profile.edit') }}
          </Button>
        </div>

        <div class="grid gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-300 mb-2">
              {{ $t('profile.username') }}
            </label>
            <input
              v-if="isEditing"
              v-model="editForm.username"
              type="text"
              class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white-custom focus:outline-none focus:ring-2 focus:ring-light-green-custom"
            />
            <p v-else class="text-white-custom">{{ user?.username }}</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-300 mb-2">
              {{ $t('profile.email') }}
            </label>
            <p class="text-white-custom">{{ user?.email }}</p>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-300 mb-2">
              {{ $t('profile.memberSince') }}
            </label>
            <p class="text-white-custom">{{ new Date(user?.createdAt || '').toLocaleDateString() }}</p>
          </div>
        </div>

        <div v-if="isEditing" class="flex gap-4 pt-4">
          <Button variant="primary" @click="saveProfile">
            {{ $t('profile.save') }}
          </Button>
          <Button variant="secondary" @click="toggleEdit">
            {{ $t('profile.cancel') }}
          </Button>
        </div>
      </div>
    </div>

    <div class="mt-8 pt-6 border-t border-gray-600">
      <Button variant="danger" @click="logout">
        {{ $t('profile.logout') }}
      </Button>
    </div>
  </div>
</template>

<style scoped>
/* Additional styles if needed */
</style>
