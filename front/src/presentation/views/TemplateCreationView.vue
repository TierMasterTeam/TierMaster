<script setup lang="ts">
import Button from '../components/base/Button.vue';
import BaseInput from '../components/base/BaseInput.vue';
import { ref } from 'vue';
const templateName = ref<string>('');
const templateCategories = ref<string[]>([]);
const currentCategory = ref<string>('');
import { CircleMinus } from 'lucide-vue-next';
import ImagePreviewInput from '../components/base/ImagePreviewInput.vue';

const addCategory = () => {
  if (currentCategory.value.trim() !== '') {
    templateCategories.value.push(currentCategory.value.trim());
    currentCategory.value = '';
  }
};

const removeCategory = (index: number) => {
  templateCategories.value.splice(index, 1);
};

const imageUploadRef = ref()

const previewImages = ref<{ name: string; url: string }[]>([])
const showNameBubbles = ref<boolean[]>([])

const triggerAddToPreview = () => {
  const refValue = imageUploadRef.value
  if (refValue && refValue.getImageData) {
    const imageData = refValue.getImageData()
    if (imageData) {
      previewImages.value.push(imageData)
      showNameBubbles.value.push(false) //add a state for the new image
      refValue.removeImage()
    }
  }
}

// show bubble with image name
const showNameBubble = (idx: number) => {
  showNameBubbles.value[idx] = true
  setTimeout(() => {
    showNameBubbles.value[idx] = false
  }, 2000)
}

const removePreviewImage = (index: number) => {
  previewImages.value.splice(index, 1)
  showNameBubbles.value.splice(index, 1)
}
</script>

<template>
  <main class="p-16">
    <label for="templateName">
      <h1 class="text-[40px] font-jersey">Tierlist Title</h1>
    </label>
    <BaseInput id="templateName" type="text" placeholder="My Tierlist title ..." class="mb-4 w-1/3 max-w-2xl"
      v-model="templateName" />
    <label for="category">
      <h2 class="text-2xl font-jersey">Add #categories</h2>
    </label>
    <form class="flex gap-2" @submit.prevent="addCategory">
      <BaseInput id="category" type="text" placeholder="category ..." class="mb-4 max-w-sm block"
        v-model="currentCategory" />
      <Button variant="secondary" type="submit" size="md" icon="plus" class="mb-4">
      </Button>
    </form>
    <ul class="flex flex-wrap pb-4 max-w-xl gap-4">
      <li v-for="(category, index) in templateCategories" :key="index" class="relative">
        <span class="bg-gray-custom rounded-full px-4 py-2 font-normal text-base font-roboto">{{ category }}</span>
        <button @click="removeCategory(index)"
          class="absolute w-6 h-6 rounded-full bg-light-gray-custom -top-2 -right-3 z-1 flex cursor-pointer">
          <CircleMinus />
        </button>
      </li>
    </ul>
    <div class="sm:flex-col flex md:flex-row gap-16">
      <div>
        <ImagePreviewInput ref="imageUploadRef" />
        <Button variant="primary" type="button" size="md" icon="image-up" @click="triggerAddToPreview" class="mt-4">
          Add to the tierlist
        </Button>
      </div>
      <div>
        <div v-if="previewImages.length > 0" class="flex flex-wrap border-2 rounded-xl p-4 gap-4 max-w-5xl">
          <div
            v-for="(img, idx) in previewImages"
            :key="idx"
            class="flex flex-col items-center relative cursor-pointer"
            @click="showNameBubble(idx)"
          >
            <img :src="img.url" :alt="img.name" class="w-19 h-19 object-cover rounded-md" />
            <button
              @click.stop="removePreviewImage(idx)"
              class="absolute w-6 h-6 rounded-full bg-light-gray-custom -top-2 -right-3 z-10 flex items-center justify-center cursor-pointer border-none"
              aria-label="Remove image"
            >
              <CircleMinus />
            </button>
            <div v-if="showNameBubbles[idx]" class="name-bubble">
              {{ img.name }}
            </div>
          </div>
        </div>
        <div v-else>
          No images added yet.
        </div>
      </div>
    </div>
    <div class="flex pt-16">
      <Button variant="secondary" type="button" size="lg" class="mt-4">
        Save Template
      </Button>
      <Button variant="primary" type="button" size="lg" class="mt-4 ml-4">
        Publish Template
      </Button>
    </div>
  </main>
</template>

<style scoped>
.name-bubble {
  position: absolute;
  top: -40px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.875rem;
  white-space: nowrap;
  z-index: 20;
  animation: fadeIn 0.2s ease-out;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}

.name-bubble::after {
  content: '';
  position: absolute;
  bottom: -5px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 5px 5px 0;
  border-style: solid;
  border-color: rgba(0, 0, 0, 0.8) transparent transparent;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translate(-50%, 5px);
  }

  to {
    opacity: 1;
    transform: translate(-50%, 0);
  }
}
</style>
