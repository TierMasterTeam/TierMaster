<script setup lang="ts">
import Button from '../components/base/Button.vue';
import BaseInput from '../components/base/BaseInput.vue';
import { ref, onMounted } from 'vue';
import { CircleMinus, Plus } from 'lucide-vue-next';
import ImagePreviewInput from '../components/base/ImagePreviewInput.vue';
import { VueDraggable } from 'vue-draggable-plus';
import GradeInput from './GradeInput.vue';
import { useTierListStore } from '../stores/tierListStore';
import type { Card, TierList } from '@/domain/interfaces/TierList';
import { useAuthStore } from '../stores/authStore';
import { useRoute } from 'vue-router';
import { useUtilsStore } from '../stores/utilsStore';
import pp from '../../assets/pp.png'; // Placeholder image import
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const showToast = useUtilsStore().showToast;
const tierListStore = useTierListStore();
const authStore = useAuthStore();
const user = authStore.user;
const route = useRoute();

const template = ref<TierList | undefined>(undefined);
const currentCategory = ref<string>('');
const imageUploadRef = ref();
const showNameBubbles = ref<boolean[]>([]);

const coverImgUrl = ref<string>(''); // Placeholder image URL

onMounted(async () => {
  const id = route.params.id;
  if (id) {
    try {
      template.value = await tierListStore.getTierListById(id as string);
    } catch (error) {
      console.error('Error fetching tier list:', error);
    }
  }
});

const onCoverFileChange = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files[0]) {
    const file = target.files[0];
    try {
      const formData = new FormData();
      formData.append('coverImage', file);
      const res = await tierListStore.uploadImages(formData);
      if (res) {
        // template.value!.coverImage = res; // backend retourne une URL
        showToast(t('editTemplate.coverSuccess'), 'success');
      }
    } catch (error) {
      showToast(t('editTemplate.coverError'), 'error');
      console.error('Error uploading cover image:', error);
    }
  }
};

const addCategory = () => {
  if (currentCategory.value.trim() !== '') {
    template.value?.tags.push(currentCategory.value.trim());
    currentCategory.value = '';
  }
};

const removeCategory = (index: number) => {
  template.value?.tags.splice(index, 1);
};

const triggerAddToPreview = async () => {
  const refValue = imageUploadRef.value;
  if (refValue && refValue.getCardData) {
    const imageData = refValue.getCardData();

    if (imageData && imageData.image instanceof File) {
      const formData = new FormData();
      formData.append('images', imageData.image);

      const res = await tierListStore.uploadImages(formData);
      template.value!.cards.push({
        name: imageData.name,
        image: res![0], // backend retourne une URL
      });
      showNameBubbles.value.push(false);
      await SaveTemplate();
      refValue.removeCard();
    }
  }
};

const showNameBubble = (idx: number) => {
  showNameBubbles.value[idx] = true;
  setTimeout(() => {
    showNameBubbles.value[idx] = false;
  }, 2000);
};

const removePreviewImage = (index: number) => {
  template.value!.cards.splice(index, 1);
  showNameBubbles.value.splice(index, 1);
};

const addGrade = () => {
  template.value!.grades.push({
    name: 'New',
    color: '#CCCCCC',
    cards: [],
  });
};

const SaveTemplate = async () => {
  if (!user?.id) {
    console.error('User not authenticated');
    return;
  }

  try {
    await tierListStore.updateTemplate(template.value!);
    showToast(t('editTemplate.updateSuccess'), 'success');
  } catch (error) {
    showToast(t('editTemplate.updateError'), 'error');
    console.error('Error updating template:', error);
  }
};
</script>


<template>
  <main class="p-16" v-if="template">
    <div class="flex w-full justify-between flex-col md:flex-row">
      <div class="w-full">
        <label for="templateName">
          <h1 class="text-[40px] font-jersey">{{ $t('editTemplate.title') }}</h1>
        </label>
        <BaseInput
          id="templateName"
          type="text"
          :placeholder="$t('editTemplate.titlePlaceholder')"
          class="mb-4 min-w-sm w-1/3 max-w-2xl"
          v-model="template.name"
        />

        <label for="category">
          <h2 class="text-2xl font-jersey">{{ $t('editTemplate.addCategories') }}</h2>
        </label>
        <form class="flex gap-2" @submit.prevent="addCategory">
          <BaseInput
            id="category"
            type="text"
            :placeholder="$t('editTemplate.categoryPlaceholder')"
            class="mb-4 max-w-sm block"
            v-model="currentCategory"
          />
          <Button variant="secondary" type="submit" size="md" class="mb-4 h-11">
            <template #icon>
              <Plus class="w-5 h-5" />
            </template>
          </Button>
        </form>

        <ul class="flex flex-wrap pb-4 max-w-xl gap-4 mb-4">
          <li v-for="(category, index) in template.tags" :key="index" class="relative">
            <span class="bg-gray-custom rounded-full px-4 py-2 font-normal text-base font-roboto">
              {{ category }}
            </span>
            <button
              @click="removeCategory(index)"
              class="absolute w-6 h-6 rounded-full bg-light-gray-custom -top-2 -right-3 z-1 flex cursor-pointer"
            >
              <CircleMinus />
            </button>
          </li>
        </ul>

        <div class="flex-col-reverse flex lg:flex-row gap-16">
          <div>
            <ImagePreviewInput ref="imageUploadRef" />
            <Button
              variant="primary"
              type="button"
              size="md"
              icon="image-up"
              @click="triggerAddToPreview"
              class="mt-4"
            >
              {{ $t('editTemplate.addToTierlist') }}
            </Button>
          </div>

          <div class="flex w-full justify-between gap-16">
            <div
              v-if="template.cards.length > 0"
              class="flex flex-wrap border-2 rounded-xl p-4 w-fit h-fit gap-4 max-w-3xl"
            >
              <div
                v-for="(card, idx) in template.cards"
                :key="idx"
                class="flex flex-col items-center relative cursor-pointer"
                @click="showNameBubble(idx)"
              >
                <img :src="card.image" :alt="card.name" class="w-19 h-19 object-cover rounded-md" />
                <button
                  @click.stop="removePreviewImage(idx)"
                  class="absolute w-6 h-6 rounded-full bg-light-gray-custom -top-2 -right-3 z-10 flex items-center justify-center cursor-pointer border-none"
                  :aria-label="$t('editTemplate.removeImage')"
                >
                  <CircleMinus />
                </button>
                <div v-if="showNameBubbles[idx]" class="name-bubble">
                  {{ card.name }}
                </div>
              </div>
            </div>
            <div v-else>{{ $t('editTemplate.noImages') }}</div>
          </div>
        </div>
      </div>

      <div class="flex flex-col items-center">
        <div>
          <label for="coverImageInput" class="w-100 h-50 block mb-12">
            <h3 class="text-2xl font-jersey">{{ $t('editTemplate.cover') }}</h3>
            <img v-if="coverImgUrl" :src="coverImgUrl" :alt="$t('editTemplate.coverAlt')" class="w-full h-full object-cover rounded-md border-white-custom border-2" />
            <div v-else class="w-full h-full flex items-center justify-center bg-light-gray-custom rounded-md mb-4 border-white-custom border-2 text-gray-500">
              {{ $t('editTemplate.noCover') }}
            </div>
          </label>
          <input type="file" accept="image/*" @change="onCoverFileChange" class="hidden" id="coverImageInput" />
        </div>

        <div class="flex gap-4 items-center justify-center">
          <h3 class="text-[32px] font-jersey">{{ $t('editTemplate.grades') }}</h3>
          <Button variant="secondary" type="button" @click="addGrade">
            <template #icon>
              <Plus class="w-5 h-5" />
            </template>
          </Button>
        </div>
        <div class="flex flex-col items-center justify-center w-fit h-fit border-2 border-white-custom rounded-xl p-4 mt-2">
          <VueDraggable v-model="template.grades" item-key="id" group="grades" class="flex-1 flex flex-col gap-2 rounded-md items-center">
            <GradeInput
              v-for="(grade, idx) in template.grades"
              :key="idx"
              v-model="grade.name"
              :color="grade.color"
              @update:color="val => grade.color = val"
              @delete="template.grades.splice(idx, 1)"
            />
          </VueDraggable>
        </div>
      </div>
    </div>

    <div class="flex pt-16">
      <Button variant="secondary" type="button" size="lg" class="mt-4" @click="SaveTemplate()">
        {{ $t('editTemplate.save') }}
      </Button>
      <Button variant="primary" type="button" size="lg" class="mt-4 ml-4" @click="SaveTemplate()">
        {{ $t('editTemplate.publish') }}
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
