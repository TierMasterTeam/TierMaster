<script setup lang="ts">
import { CircleMinus, Image } from 'lucide-vue-next'
import { ref } from 'vue'
import BaseInput from './BaseInput.vue'
import type { Card } from '@/domain/interfaces/TierList'

const file = ref<File | null>(null)
const imageName = ref('')
const previewUrl = ref<string | null>(null)

const onFileChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files[0]) {
    file.value = target.files[0]
    previewUrl.value = URL.createObjectURL(target.files[0])
  }
}

const removeCard = () => {
  file.value = null
  imageName.value = ''
  previewUrl.value = null
}

// Function to get image data
const getCardData = ():Card | null => {
  if (file.value && previewUrl.value && imageName.value) {
    return {
      name: imageName.value,
      image: previewUrl.value,
    }
  }
  return null
}

defineExpose({ file, imageName, previewUrl, getCardData, removeCard })
</script>

<template>
  <div class="relative flex flex-col border-2 border-white-custom rounded-xl w-[300px]">
    <label class="cursor-pointer relative z-10 w-[300px] h-[200px] flex items-center justify-center">
      <Image :size="32" class="bg-gray-custom/30 rounded-md backdrop-blur-3xl" />
      <input
        type="file"
        accept="image/*"
        @change="onFileChange"
        class="hidden"
      />
    </label>
    <button v-if="file" @click="removeCard()" class="top-1 right-1 absolute z-10 bg-red-custom rounded-full cursor-pointer">
        <CircleMinus />
    </button>
    <BaseInput
      type="text"
      v-model="imageName"
      placeholder="Image name"
      class="rounded-t-none h-[96px] text-lg"
    />
    <div v-if="previewUrl" class="absolute rounded-t-[10px] overflow-hidden w-[296px] h-[200px]">
      <img :src="previewUrl" alt="Preview" class="h-full w-full object-cover" />
    </div>
  </div>
</template>