<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useTierListStore } from '../stores/tierListStore';
import { type Tier } from '@/domain/interfaces/TierList';
import { type Item } from '@interfaces/Item';

import ItemCard from '@components/ItemCard.vue';

const tierListStore = useTierListStore();

const draggedItem = ref<Item | null>(null);
const draggedFromTier = ref<Tier | null>(null);
const dragOverIndex = ref<number>(-1);
const dragOverTier = ref<Tier | null>(null);
const isDragging = ref(false);

const onDragStart = (item: Item, tier: Tier, event: DragEvent) => {
  draggedItem.value = item;
  draggedFromTier.value = tier;
  isDragging.value = true;

  // Set a custom drag image if supported
  if (event.dataTransfer && event.target instanceof HTMLElement) {
    // Create a ghost image for better drag visuals
    const ghostEl = event.target.cloneNode(true) as HTMLElement;
    ghostEl.style.opacity = '0.7';
    document.body.appendChild(ghostEl);
    event.dataTransfer.setDragImage(ghostEl, 15, 15);

    // Clean up after drag starts
    setTimeout(() => {
      document.body.removeChild(ghostEl);
    }, 0);
  }
};

const onDragOver = (event: DragEvent, index: number, tier: Tier) => {
  event.preventDefault();
  dragOverIndex.value = index;
  dragOverTier.value = tier;
};

const onDragLeave = () => {
  dragOverIndex.value = -1;
};

const onDrop = (targetTier: Tier, targetIndex: number = -1) => {
  if (draggedItem.value && draggedFromTier.value) {
    const fromIndex = draggedFromTier.value.items.findIndex(
      (item: Item) => item.name === draggedItem.value!.name
    );

    if (fromIndex !== -1) {
      draggedFromTier.value.items.splice(fromIndex, 1);
    }

    if (targetIndex >= 0) {
      if (targetTier === draggedFromTier.value && targetIndex > fromIndex) {
        targetIndex--;
      }
      targetTier.items.splice(targetIndex, 0, draggedItem.value);
    } else {
      targetTier.items.push(draggedItem.value);
    }

    draggedItem.value = null;
    draggedFromTier.value = null;
    dragOverIndex.value = -1;
    dragOverTier.value = null;
    isDragging.value = false;
  }
};

const onDragEnd = () => {
  // Reset drag states
  draggedItem.value = null;
  draggedFromTier.value = null;
  dragOverIndex.value = -1;
  dragOverTier.value = null;
  isDragging.value = false;
};

onMounted(() => {
  tierListStore.connectWebSocket();
});

onUnmounted(() => {
  tierListStore.disconnectWebSocket();
});
</script>

<template>
  <div class="container mx-auto p-4">
    <h1 v-if="tierListStore.tierList.name" class="text-4xl font-bold text-[#31E7C3] pb-4">
      {{ tierListStore.tierList.name + ' :' }}
    </h1>
    <div class="grid gap-4">
      <div
        v-for="tier in tierListStore.tierList.tiers"
        :key="tier.name"
        :class="[
          'p-3 rounded-3xl shadow-md tier-container flex gap-2 border-2',
          { 'tier-dropzone': isDragging, 'tier-active-dropzone': isDragging && dragOverTier === tier && dragOverIndex === -1 }
        ]"
        @dragover.prevent
        @drop.prevent="onDrop(tier)"
        @dragenter.prevent="dragOverTier = tier">
        <div :style="{backgroundColor: tier.color}" class="text-center text-zinc-900 text-2xl mb-2 w-19 h-19 rounded-full flex items-center justify-center">{{ tier.name }}</div>
        <div
          :class="[
            'flex flex-wrap gap-2 rounded-md',
          ]"
          :style="{ transition: 'background-color 0.2s ease' }">

          <ItemCard
            v-for="(item, index) in tier.items"
            :key="`${item.name}-${index}`"
            :item="item"
            :tier="tier"
            :index="index"
            :isDragging="isDragging"
            :isDraggedItem="draggedItem?.name === item.name"
            :isDropTarget="dragOverIndex === index && dragOverTier === tier"
            @dragstart="onDragStart"
            @dragover="onDragOver"
            @dragleave="onDragLeave"
            @drop="onDrop"
            @dragend="onDragEnd"
          />

        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.tier-container {
  transition: box-shadow 0.3s ease, border 0.3s ease;
}

.tier-dropzone {
  box-shadow: 0 0 0 2px rgba(255, 4, 4, 0.05);
}

.tier-active-dropzone {
  box-shadow: 0 0 0 3px #1d787478;
  animation: glow 2s infinite;
}

@keyframes glow {
  0% { box-shadow: 0 0 0 2px rgba(49, 231, 195, 0.3); }
  50% { box-shadow: 0 0 0 4px rgba(29, 120, 116, 0.5); }
  100% { box-shadow: 0 0 0 2px rgba(49, 231, 195, 0.3); }
}
</style>

