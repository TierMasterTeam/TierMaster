<script setup lang="ts">
import { onMounted, ref } from 'vue';
import ConfirmPopup from '../components/base/ConfirmPopup.vue';
import Button from '../components/base/Button.vue';
import router from '../router';
import { useTierListStore } from '../stores/tierListStore';
import type { Template, TierList } from '@/domain/interfaces/TierList';
import { useAuthStore } from '../stores/authStore';

const authStore = useAuthStore();
const user = authStore.user;

const tierListStore = useTierListStore();

const templates = ref<Template[]>([]);

const showDeletePopup = ref(false);
const templateToDelete = ref<Template | null>(null);
const confirmVariant = ref<'danger' | 'primary' | 'secondary'>('danger');

onMounted(async () => {
    if (!user) {
        console.error('User is not authenticated');
        return;
    }
    try {
        templates.value = await tierListStore.getUserTemplates(user.id);
    } catch (error) {
        console.error('Error loading templates:', error);
    }
});

const createTempateAction = async() => {
    // Logic to create a new template
    console.log('Creating new template....');
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

function getUntitledIndex(template: Template, index: number): number {
    if (template.name) return 0;
    // Count how many untitled templates exist before this one
    let count = 0;
    for (let i = 0; i < templates.value.length; i++) {
        if (!templates.value[i].name && i <= index) {
            count++;
        }
    }
    return count;
}

function openDeletePopup(template: Template) {
  templateToDelete.value = template;
  showDeletePopup.value = true;
}

function closeDeletePopup() {
  showDeletePopup.value = false;
  templateToDelete.value = null;
}

async function confirmDelete() {
  if (templateToDelete.value) {
    try {
      await tierListStore.deleteTemplate(templateToDelete.value.id!)
      templates.value = templates.value.filter(t => t.id !== templateToDelete.value?.id)
    } catch (error) {
      console.error('Error deleting template:', error)
    }
  }
  closeDeletePopup()
}

</script>

<template>
    <div class="flex flex-col p-16 w-full">
        <h1 class="text-[40px] font-jersey">
            {{ $t('templates.title') }}
        </h1>
        <Button size="md" variant="primary" class="mt-4 w-fit" @click="createTempateAction">
            {{ $t('templates.createNew') }}
        </Button>
        <ul class="flex w-full flex-col mt-4 border-2 border-white-custom">
            <li v-for="(template, index) in templates" :key="template.id || index" class="flex items-center w-full justify-between p-2 border-b-2 border-white-custom">
                <div>
                    {{ template.name || $t('templates.untitled', { n: getUntitledIndex(template, index) }) }}
                </div>
                <div class="flex gap-2">
                    <Button size="sm" variant="primary" @click="() => router.push({ name: 'myTemplate', params: { id: template.id } })">
                        {{ $t('templates.edit') }}
                    </Button>
                    <Button size="sm" variant="danger" @click="() => openDeletePopup(template)">
                        {{ $t('templates.delete') }}
                    </Button>
                </div>
            </li>
        </ul>
        <ConfirmPopup
            :show="showDeletePopup"
            :confirmVariant="confirmVariant"
            @confirm="confirmDelete"
            @cancel="closeDeletePopup"
        >
            <template #title>
                <h2 class="text-xl font-bold mb-4">Confirmer la suppression</h2>
            </template>
            <template #content>
                <p class="mb-6">Êtes-vous sûr de vouloir supprimer ce modèle&nbsp;?</p>
            </template>
        </ConfirmPopup>
    </div>
</template>



