<script setup lang="ts">
import { defineProps, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import type { Template, TierList } from '@/domain/interfaces/TierList';
import { useTierListStore } from '../stores/tierListStore';
import { useAuthStore} from '../stores/authStore';

const tierlistStore = useTierListStore();
const authStore= useAuthStore();

interface Props {
  template: Template;
}

const props = defineProps<Props>();
const router = useRouter();

const author = authStore.user?.id

const getTemplateImage = (template: Template) => {
  if (template.imgCover) {
    return template.imgCover;
  }
  return template.cards && template.cards.length > 0 && template.cards[0].image
    ? template.cards[0].image
    : '/default-image.png';
};

const createTierlistByTemplate = async(template: Template) => {

  if (!authStore.isLoggedIn && !author) {
    console.error('User is not authenticated');
    router.push({ name: 'login', query: { redirect: router.currentRoute.value.fullPath } });
    return;
  }

  const res = await tierlistStore.createTierList(
    template, author!
  )

  if (res) {
    router.push({
      name: 'myTierlists',
      params: { id: res.id }
    });
  } else {
    console.error('Failed to create a new tier list from template');
  }
};

</script>

<template>
  <button
    class="w-2xs rounded-xl border-2 overflow-hidden cursor-pointer hover:border-light-green-custom transition-colors duration-200"
    @click="createTierlistByTemplate(template)"
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
