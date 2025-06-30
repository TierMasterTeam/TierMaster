<script setup lang="ts">
import BaseInput from '../components/base/BaseInput.vue';
import { LayoutPanelTop, Search } from 'lucide-vue-next';
import Button from '../components/base/Button.vue';
import { onMounted, ref, computed, watch } from 'vue';
import { useTierListStore } from '../stores/tierListStore';
import type { Template } from '@/domain/interfaces/TierList';
import TemplateCard from '../components/TemplateCard.vue';
import TemplateCarousel from '../components/TemplateCarousel.vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/authStore';

const router = useRouter();
const authStore = useAuthStore();
const tierListStore = useTierListStore();

const templates = ref<Template[]>([]);
const searchQuery = ref('');
const CarouselTemplates = ref<Template[]>([]);

const resultNb = computed(() => templates.value.length);

const searchTemplates = async () => {
  const result = await tierListStore.searchTemplates(searchQuery.value, 1, 100);
  templates.value = result.data || [];
};

let debounceTimeout: ReturnType<typeof setTimeout>;
let isFirstSearch = true;

watch(searchQuery, (newQuery) => {
  clearTimeout(debounceTimeout);
  if (newQuery.trim()) {
    if (isFirstSearch) {
      isFirstSearch = false;
      searchTemplates();
    } else {
      debounceTimeout = setTimeout(() => {
        searchTemplates();
      }, 300);
    }
  } else {
    isFirstSearch = true;
    templates.value = [];
  }
});


const createTemplateAction = async () => {
  const id = await tierListStore.initTemplate();
  if (id) {
    router.push({
      name: 'myTemplate',
      params: { id },
    });
  } else {
    console.error('Échec lors de la création d’un nouveau template');
  }
};

onMounted(async () => {
  try {
    CarouselTemplates.value = await tierListStore.getPublicTemplates();
  } catch (error) {
    console.error('Erreur lors du chargement des templates populaires:', error);
  }
});
</script>

<template>
  <div class="flex flex-col px-4 items-center w-full">
    <div class="w-fit flex flex-col">
      <h1 class="font-jersey pt-16 text-6xl md:text-[80px] tracking-widest text-light-green-custom">
        TierMaster
      </h1>

      <div class="flex flex-col md:flex-row w-full justify-between gap-4">
        <BaseInput
          v-model="searchQuery"
          :label="$t('home.searchForTemplate')"
          :placeholder="$t('home.searchPlaceholder')"
          class="max-w-[39rem] flex-grow"
          id="searchInput"
        >
          <template #right>
            <label for="base-input" class="cursor-pointer flex items-center">
              <Search class="h-5 w-5 text-light-gray-custom" />
            </label>
          </template>
        </BaseInput>

        <Button variant="primary" class="w-fit" @click="createTemplateAction">
          <template #icon>
            <LayoutPanelTop class="w-5 h-5" />
          </template>
          {{ $t('home.createYourTemplate') }}
        </Button>
      </div>

      <div v-if="searchQuery === ''" class="pt-8 pb-4">
        <h2 class="font-jersey tracking-widest text-[2.5rem] pb-4">
          {{ $t('home.popularTemplates') }}
        </h2>
        <TemplateCarousel :templates="CarouselTemplates" />

        <h2 class="font-jersey tracking-widest text-[2.5rem] pt-16">
          {{ $t('home.whatsNew') }}
        </h2>
        <p class="max-w-[740px] px-4 pb-16">
          {{ $t('home.whatsNewDesc') }}
        </p>

        <h2 class="font-jersey tracking-widest text-[2.5rem] pb-4">
          {{ $t('home.recentTemplates') }}
        </h2>
        <TemplateCarousel :templates="CarouselTemplates" class="pb-32" />
      </div>

      <div v-else>
        <div class="pt-8 pb-4">
          {{ resultNb }} {{ $t('home.results') }}
        </div>
        <div
          v-if="templates.length > 0"
          class="grid w-fit grid-cols-1 grid2:grid-cols-2 grid3:grid-cols-3 grid4:grid-cols-4 gap-4"
        >
          <TemplateCard
            v-for="template in templates"
            :key="template.id"
            :template="template"
          />
        </div>
        <div
          v-else
          class="text-center py-8 text-light-gray-custom min-w-[284px] grid2:min-w-[592px] grid3:min-w-[896px] grid4:min-w-[1200px]"
        >
          {{ $t('home.noTemplatesFound', { query: searchQuery }) }}
        </div>
      </div>
    </div>
  </div>
</template>
