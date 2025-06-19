<script setup lang="ts">
import BaseInput from '../components/base/BaseInput.vue';
import { LayoutPanelTop, Search } from 'lucide-vue-next';
import Button from '../components/base/Button.vue';
import { onMounted, ref, computed } from 'vue';
import { useTierListStore } from '../stores/tierListStore';
import type { TierList } from '@/domain/interfaces/TierList';
import TemplateCard from '../components/TemplateCard.vue';
import TemplateCarousel from '../components/TemplateCarousel.vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/authStore';

const router = useRouter();

const authStore = useAuthStore();
const templates = ref<TierList[]>([]);
const searchQuery = ref('');

const tierListStore = useTierListStore();

// Filtrer les templates en fonction de la recherche
const filteredTemplates = computed(() => {
  if (!searchQuery.value.trim()) return templates.value;
  
  const query = searchQuery.value.toLowerCase();
  return templates.value.filter(template => 
    template.name.toLowerCase().includes(query)
  );
});

// Nombre de résultats après filtrage
const resultNb = computed(() => filteredTemplates.value.length);

onMounted(async() => {
  templates.value = await tierListStore.getPublicTemplates();
});

const createTempateAction = async() => {
  if (!authStore.isLoggedIn) {
    console.error('User is not authenticated');
    router.push({ name: 'login', query: { redirect: router.currentRoute.value.fullPath } });
    return;
  }
    const id = await tierListStore.initTemplate();
    if (id) {
        router.push({
            name: 'myTemplate',
            params: { id }
        });
    } else {
        console.error('Failed to create a new template');
    }

    
};

</script>
<template>
  <div class="flex flex-col px-4 items-center">
    <div class="w-fit flex flex-col">
      <h1 class="font-jersey pt-16 text-[80px] tracking-widest text-light-green-custom">TierMaster</h1>
      <div class="flex flex-col md:flex-row w-full justify-between">
        <BaseInput 
          v-model="searchQuery"
          label="Search for a template" 
          placeholder="Search..." 
          class="max-w-[39rem] w-full" 
          id="searchInput"
        >
          <template #right>
            <label for="base-input" class="cursor-pointer flex items-center">
              <Search class="h-5 w-5 text-light-gray-custom" />
            </label>
          </template>
        </BaseInput>
        <Button variant="primary" class="w-fit" @click="createTempateAction">
          <template #icon>
            <LayoutPanelTop class="w-5 h-5" />
          </template>
          Create your template
        </Button>
      </div>

      
      <div v-if="searchQuery === ''" class="pt-8 pb-4">
        <h2 class="font-jersey tracking-widest text-[2.5rem] pb-4">Popular Templates</h2>
        <TemplateCarousel
          :templates="templates"
        />

        <h2 class="font-jersey tracking-widest text-[2.5rem] pt-16">What's New</h2>
        <p class="max-w-[740px] px-4 pb-16">Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.</p>
        <h2 class="font-jersey tracking-widest text-[2.5rem] pb-4">Recent Templates</h2>
        <TemplateCarousel
          :templates="templates"
          class="pb-32"
        />
      </div>

      <div v-else>
        <div class="pt-8 pb-4">{{ resultNb }} results</div>
        <div v-if="searchQuery !== '' && filteredTemplates.length > 0" class="grid w-fit grid-cols-1 grid2:grid-cols-2 grid3:grid-cols-3 grid4:grid-cols-4 gap-4">
          <TemplateCard
            v-for="template in filteredTemplates" 
            :key="template.id"
            :template="template"
          />
        </div>
        <div v-else class="text-center py-8 text-light-gray-custom min-w-[284px] grid2:min-w-[592px] grid3:min-w-[896px] grid4:min-w-[1200px]">
          No templates found matching "{{ searchQuery }}"
        </div>
      </div>
    </div>
  </div>
</template>
