<script lang="ts" setup>
import { ref, watch } from 'vue';
import { type Tier } from '@/domain/interfaces/TierList';
import { type Item } from '@interfaces/Item';

const props = defineProps<{
  item: Item;
  tier: Tier;
  isDragging?: boolean;
}>();

const showNameBubble = ref(false);
const timeoutId = ref<number | null>(null);

const toggleNameBubble = () => {
  if (props.isDragging) return; // Désactive pendant le drag
  showNameBubble.value = true;
  if (timeoutId.value !== null) {
    window.clearTimeout(timeoutId.value);
  }
  timeoutId.value = window.setTimeout(() => {
    showNameBubble.value = false;
  }, 2000);
};

// Masque la bulle si drag commence
watch(() => props.isDragging, (val) => {
  if (val) showNameBubble.value = false;
});
</script>

<template>
  <div
    class="w-19 h-19 rounded-md flex items-center justify-center bg-cover bg-center bg-no-repeat
 text-white font-bold relative transition-transform item-card"
    :style="`background-image: url(${item.img});`"
    @click="toggleNameBubble"
  >
    <div
      v-if="showNameBubble"
      class="name-bubble"
      @click.stop
    >
      {{ item.name }}
    </div>
  </div>
</template>

<style scoped>
.item-card {
  cursor: grab;
  user-select: none;
  transition: transform 0.15s ease, box-shadow 0.15s ease;
  position: relative;
}

.item-card:active {
  cursor: grabbing;
  transform: scale(1.05);
}

@keyframes pulse {
  0% { opacity: 0.6; }
  50% { opacity: 1; }
  100% { opacity: 0.6; }
}

.name-bubble {
  position: absolute;
  top: -40px;
  left: 50%;
  transform: translateX(-50%);
  background-color: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.875rem;
  white-space: nowrap;
  z-index: 20;
  animation: fadeIn 0.2s ease-out;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}

.name-bubble::after {
  content: '';
  position: absolute;
  bottom: -5px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 5px 5px 0;
  border-style: solid;
  border-color: rgba(0, 0, 0, 0.8) transparent transparent;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translate(-50%, 5px);
  }
  to {
    opacity: 1;
    transform: translate(-50%, 0);
  }
}
</style>
