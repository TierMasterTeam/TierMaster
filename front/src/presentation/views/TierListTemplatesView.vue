<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Button from '../components/base/Button.vue';
import router from '../router';
import { useTierListStore } from '../stores/tierListStore';
import type { TierList } from '@/domain/interfaces/TierList';
import { useAuthStore } from '../stores/authStore';

const authStore = useAuthStore();
const user = authStore.user;

const tierListStore = useTierListStore();

const templates = ref<TierList[]>([]);

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

const getUntitledIndex = (template: TierList, index: number): number => {
    if (template.name) return 0;
    
    // Count how many untitled templates exist before this one
    let count = 0;
    for (let i = 0; i < templates.value.length; i++) {
        if (!templates.value[i].name && i <= index) {
            count++;
        }
    }
    return count;
};

</script>

<template>
    <div class="flex flex-col p-16">
        <h1 class="text-[40px] font-jersey">
            TierList templates
        </h1>
        <Button size="md" variant="primary" class="mt-4 w-fit" @click="createTempateAction">
            Create new template
        </Button>
        <ul class="flex flex-col mt-4 border-2 border-white-custom">
            <li v-for="(template, index) in templates" class="flex items-center justify-between p-2 border-b-2 border-white-custom">
                <div>
                    {{ template.name || `Untitled Template ${getUntitledIndex(template, index)}` }}
                </div>
                <div class="flex gap-2">
                    <Button size="sm" variant="primary" @click="() => router.push({ name: 'myTemplate', params: { id: template.id } })">
                        Edit
                    </Button>
                    <Button size="sm" variant="danger" @click="() => console.log('Delete template', template.id)">
                        Delete
                    </Button>
                </div>
            </li>
        </ul>
    </div>

</template>