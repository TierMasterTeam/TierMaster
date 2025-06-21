<script setup lang="ts">
import { defineProps } from 'vue';
import { useRouter } from 'vue-router';
import type { TierList } from '@/domain/interfaces/TierList';

interface Props {
  template: TierList;
}

const props = defineProps<Props>();
const router = useRouter();

const getTemplateImage = (template: TierList) => {
  if (template.imgCover) {
    return template.imgCover;
  }
  return template.cards && template.cards.length > 0 && template.cards[0].image
    ? template.cards[0].image
    : '/default-image.png';
};

const navigateToTemplate = () => {
  router.push(`/tierlist/${props.template.id}`);
};
</script>

<template>
  <button 
    class="w-2xs rounded-xl border-2 overflow-hidden cursor-pointer hover:border-light-green-custom transition-colors duration-200"
    @click="navigateToTemplate"
  >
    <img 
      :src="getTemplateImage(template)" 
      :alt="template.name" 
      class="h-48 w-full object-cover bg-gray-50"
    >
    <div class="h-25 border-t-2 flex items-center justify-center p-4">
      {{ template.name }}
    </div>
  </button>
</template>
