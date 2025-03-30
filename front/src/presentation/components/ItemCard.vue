<script lang="ts" setup>
import { ref } from 'vue';
import { type Tier } from '@/domain/interfaces/TierList';
import { type Item } from '@interfaces/Item';

const props = defineProps<{
  item: Item;
  tier: Tier;
  index: number;
  isDragging: boolean;
  isDraggedItem: boolean;
  isPreview?: boolean;
}>();

const emit = defineEmits<{
  'dragstart': [item: Item, tier: Tier, event: DragEvent];
  'dragover': [event: DragEvent, index: number, tier: Tier];
  'dragleave': [];
  'drop': [tier: Tier, index: number];
  'dragend': [];
}>();

// State for the name bubble
const showNameBubble = ref(false);
const timeoutId = ref<number | null>(null);

const onDragStart = (event: DragEvent) => {
  emit('dragstart', props.item, props.tier, event);
};

const onDragOver = (event: DragEvent) => {
  emit('dragover', event, props.index, props.tier);
};

const onDrop = (event: DragEvent) => {
  event.preventDefault();
  event.stopPropagation();
  emit('drop', props.tier, props.index);
};

const onDragLeave = () => {
  emit('dragleave');
};

const onDragEnd = () => {
  emit('dragend');
};

const toggleNameBubble = () => {
  // Only show bubble if not currently dragging
  if (props.isDragging) return;

  showNameBubble.value = true;

  // Clear any existing timeout
  if (timeoutId.value !== null) {
    window.clearTimeout(timeoutId.value);
  }

  // Auto-hide the bubble after 2 seconds
  timeoutId.value = window.setTimeout(() => {
    showNameBubble.value = false;
  }, 2000);
};
</script>

<template>
  <div
    class="w-19 h-19 rounded-md flex items-center justify-center bg-cover bg-center bg-no-repeat
 text-white font-bold relative transition-all item-card"
    :class="{
      'dragging': isDraggedItem && !isPreview,
      'item-hover': !isDragging,
      'preview-item': isPreview
    }"
    :style="`
      background-image: url(${item.img});
      transform: ${isDraggedItem && !isPreview ? 'scale(0.9)' : 'scale(1)'};
      opacity: ${isPreview ? '0.6' : '1'};
    `"
    draggable="true"
    @dragstart="onDragStart"
    @dragover.prevent="onDragOver"
    @dragleave="onDragLeave"
    @drop.prevent="onDrop"
    @dragend="onDragEnd"
    @click="toggleNameBubble"
  >
    <!-- Name bubble -->
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
  transition: transform 0.15s ease, box-shadow 0.15s ease, opacity 0.2s ease;
  position: relative;
}

.item-card:active {
  cursor: grabbing;
  transform: scale(1.05);
}

.item-card.dragging {
  opacity: 0;  /* Hide the original item during drag */
}

.item-card.item-hover:hover {
  transform: scale(1.05);
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  z-index: 10;
}

.item-card.preview-item {
  border: 2px dashed #31E7C3;
  box-shadow: 0 0 8px rgba(49, 231, 195, 0.4);
  animation: pulse-border 1.5s infinite;
}

@keyframes pulse-border {
  0% { border-color: rgba(49, 231, 195, 0.4); }
  50% { border-color: rgba(49, 231, 195, 0.8); }
  100% { border-color: rgba(49, 231, 195, 0.4); }
}

/* Name bubble styles */
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
