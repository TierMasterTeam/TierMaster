<script setup lang="ts">
import Button from '../components/base/Button.vue';
import { useTierListStore } from '../stores/tierListStore';

const tierListStore = useTierListStore();

const createTempateAction = () => {
    // Logic to create a new template
    console.log('Create new template action triggered');
    tierListStore.initTemplate();
};

</script>

<template>
    <div class="flex flex-col p-16">
        <h1 class="text-[40px] font-jersey">
            TierList templates
        </h1>
        <ul>
            <li>lorem ipsum</li>
            <li>lorem ipsum</li>
            <li>lorem ipsum</li>
        </ul>
    </div>

    <Button size="md" variant="primary" class="mt-4" @click="createTempateAction">
        Create new template
    </Button>
</template>