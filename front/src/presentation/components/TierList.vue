<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useTierListStore } from '../stores/tierListStore';
import { VueDraggable } from 'vue-draggable-plus';
import ItemCard from '../components/ItemCard.vue';
import Button from './base/Button.vue';
const tierListStore = useTierListStore();
const isDragging = ref(false);

onMounted(async () => {
  await tierListStore.getTierListById('680cde3fa71aefcf59820e88');
});

</script>

<template>
  <div class="container mx-auto p-4" v-if="tierListStore.currentTierlist">
    <h1 v-if="tierListStore.currentTierlist.name" class="text-4xl font-bold text-[#31E7C3] pb-4 font-jersey">
      {{ tierListStore.currentTierlist.name }} :
    </h1>

    <div class="grid gap-4">
      <div v-for="grade in tierListStore.currentTierlist.grades"
        :key="grade.name"
        class="p-3 rounded-3xl shadow-md flex gap-2 border-2"
      >
        <div
          :style="{ backgroundColor: grade.color }"
          class="text-center text-zinc-900 text-2xl w-19 h-19 rounded-full flex items-center justify-center"
        >
          {{ grade.name }}
        </div>
        <VueDraggable
          v-model="grade.cards"
          item-key="name"
          group="grades"
          class="flex-1 flex flex-wrap gap-2 rounded-md items-center"
          @start="isDragging = true"
          @end="isDragging = false"
        >
          <div v-for="card in grade.cards" :key="card.name" class="w-19 h-19">
            <ItemCard :card="card" :grade="grade" :is-dragging="isDragging"/>
          </div>
        </VueDraggable>
      </div>
      <VueDraggable
          v-model="tierListStore.currentTierlist.cards"
          item-key="name"
          group="grades"
          class="flex-1 flex flex-wrap gap-2 rounded-md items-center"
          @start="isDragging = true"
          @end="isDragging = false"
        >
          <div v-for="card in tierListStore.currentTierlist.cards" :key="card.name" class="w-19 h-19">
            <ItemCard :card="card" :is-dragging="isDragging"/>
          </div>
        </VueDraggable>
    </div>
    <Button type="button" variant="primary" size="md" class="mt-4" @click="tierListStore.saveTierList">
      Save Tierlist
    </Button>
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
