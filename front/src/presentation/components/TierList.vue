<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useTierListStore } from '../stores/tierListStore';
import { VueDraggable } from 'vue-draggable-plus';
import ItemCard from './ItemCard.vue';
const tierListStore = useTierListStore();

const isDragging = ref(false);

onMounted(() => {
  tierListStore.connectWebSocket();
});

onUnmounted(() => {
  tierListStore.disconnectWebSocket();
});
</script>

<template>
  <div class="container mx-auto p-4">
    <h1 v-if="tierListStore.tierList.name" class="text-4xl font-bold text-[#31E7C3] pb-4 font-jersey">
      {{ tierListStore.tierList.name }} :
    </h1>

    <div class="grid gap-4">
      <div
        v-for="tier in tierListStore.tierList.tiers"
        :key="tier.name"
        class="p-3 rounded-3xl shadow-md flex gap-2 border-2"
      >
        <div
          :style="{ backgroundColor: tier.color }"
          class="text-center text-zinc-900 text-2xl mb-2 w-19 h-19 rounded-full flex items-center justify-center"
        >
          {{ tier.name }}
        </div>

        <VueDraggable
          v-model="tier.items"
          item-key="name"
          group="tiers"
          class="flex-1 flex flex-wrap gap-2 rounded-md min-h-[90px]"
          @start="isDragging = true"
          @end="isDragging = false"
        >
          <div v-for="item in tier.items" :key="item.name" class="w-19 h-19">
            <ItemCard :item="item" :tier="tier" :is-dragging="isDragging"/>
          </div>
        </VueDraggable>
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
