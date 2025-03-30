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
const dragSourceIndex = ref(-1);

// Create a temporary state for preview positioning during drag
const previewState = ref({
  items: [] as Item[],
  tier: null as Tier | null,
  active: false
});


// Function to handle drag start
const onDragStart = (item: Item, tier: Tier, event: DragEvent) => {
  draggedItem.value = item;
  draggedFromTier.value = tier;
  dragSourceIndex.value = tier.items.findIndex((i: Item) => i.name === item.name);
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

  // Only proceed if we have a valid dragged item
  if (!draggedItem.value || !draggedFromTier.value) return;

  // Handle special case for end of list
  const itemCount = tier.items.length;
  // If mouse position is in the last third of the last card, place after it
  if (index === itemCount - 1) {
    const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
    const mouseX = event.clientX;
    const mouseY = event.clientY;

    // Check if mouse is in right third of the card (assuming horizontal layout)
    if (mouseX > rect.left + (rect.width * 0.7)) {
      index = itemCount;
    }
  }

  dragOverIndex.value = index;
  dragOverTier.value = tier;

  // Create a preview of the tier's items with the dragged item in the new position
  if (tier && draggedItem.value) {
    // Clone the current tier's items
    const itemsCopy = [...tier.items];

    // If the dragged item is from the same tier, remove it from its current position
    if (tier === draggedFromTier.value) {
      const currentIndex = itemsCopy.findIndex(i => i.name === draggedItem.value!.name);
      if (currentIndex !== -1) {
        itemsCopy.splice(currentIndex, 1);
      }
    }

    // Insert the dragged item at the new position
    if (index >= 0 && index <= itemsCopy.length) {
      itemsCopy.splice(index, 0, draggedItem.value);
    } else {
      itemsCopy.push(draggedItem.value);
    }

    // Update the preview state
    previewState.value = {
      items: itemsCopy,
      tier: tier,
      active: true
    };
  }
};

const onDragLeave = () => {
  // We don't reset dragOverIndex here to maintain the preview
  // This is handled when the drag ends or we drop
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

    // Reset all tracking states
    resetDragState();
  }
};

const onDragEnd = () => {
  resetDragState();
};

const resetDragState = () => {
  draggedItem.value = null;
  draggedFromTier.value = null;
  dragOverIndex.value = -1;
  dragOverTier.value = null;
  isDragging.value = false;
  dragSourceIndex.value = -1;
  previewState.value = {
    items: [],
    tier: null,
    active: false
  };
};

// Get the displayed items for a tier, taking into account the preview if active
const getDisplayedItems = (tier: Tier) => {
  if (previewState.value.active && previewState.value.tier === tier) {
    return previewState.value.items;
  }
  return tier.items;
};

// Check if an item is the one being dragged
const isItemBeingDragged = (item: Item): boolean => {
  return Boolean(draggedItem.value && draggedItem.value.name === item.name);
};

onMounted(() => {
  tierListStore.connectWebSocket();
});

onUnmounted(() => {
  tierListStore.disconnectWebSocket();
});
const onDragOverEmptyZone = (tier: Tier) => {
  if (!draggedItem.value || !draggedFromTier.value) return;

  dragOverTier.value = tier;
  dragOverIndex.value = tier.items.length; // fin de liste

  // Construire la preview à la fin
  const itemsCopy = [...tier.items];
  if (tier === draggedFromTier.value) {
    const currentIndex = itemsCopy.findIndex(i => i.name === draggedItem.value!.name);
    if (currentIndex !== -1) {
      itemsCopy.splice(currentIndex, 1);
    }
  }

  itemsCopy.push(draggedItem.value);

  previewState.value = {
    items: itemsCopy,
    tier: tier,
    active: true
  };
};

</script>

<template>
  <div class="container mx-auto p-4">
    <h1 v-if="tierListStore.tierList.name" class="text-4xl font-bold text-[#31E7C3] pb-4">
      {{ tierListStore.tierList.name + ' :' }}
    </h1>
    <div class="grid gap-4">
      <div v-for="tier in tierListStore.tierList.tiers" :key="tier.name" :class="[
        'p-3 rounded-3xl shadow-md tier-container flex gap-2 border-2',
        { 'tier-dropzone': isDragging, 'tier-active-dropzone': isDragging && dragOverTier === tier && dragOverIndex === -1 }
      ]" @drop.prevent="onDrop(tier)" @dragenter.prevent="dragOverTier = tier">
        <div :style="{ backgroundColor: tier.color }"
          class="text-center text-zinc-900 text-2xl mb-2 w-[76px] h-[76px] min-w-[76px] flex-shrink-0 rounded-full flex items-center justify-center">
          {{ tier.name }}
        </div>

        <div :class="[
          'flex flex-wrap gap-2 rounded-md w-full',
        ]" :style="{ transition: 'background-color 0.2s ease' }">

          <ItemCard v-for="(item, index) in getDisplayedItems(tier)" :key="`${item.name}-${index}`" :item="item"
            :tier="tier" :index="index" :isDragging="isDragging" :isDraggedItem="isItemBeingDragged(item)"
            :isPreview="previewState.active && previewState.tier === tier && isItemBeingDragged(item)"
            @dragstart="onDragStart" @dragover="onDragOver" @dragleave="onDragLeave" @drop="onDrop"
            @dragend="onDragEnd" />

          <!-- Drop zone de fin placée après toutes les cartes, suit le wrap -->
          <div class="flex-grow h-auto flex" @dragover.prevent="onDragOverEmptyZone(tier)" @drop.prevent="onDrop(tier)">
          </div>
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
  0% {
    box-shadow: 0 0 0 2px rgba(49, 231, 195, 0.3);
  }

  50% {
    box-shadow: 0 0 0 4px rgba(29, 120, 116, 0.5);
  }

  100% {
    box-shadow: 0 0 0 2px rgba(49, 231, 195, 0.3);
  }
}
</style>
