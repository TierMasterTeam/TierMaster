<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import type { RoomCard } from '@interfaces/RoomTierlist.ts'
import RoomUserAvatar from '@components/RoomUserAvatar.vue'
import { useTierlistRoomStore } from '@stores/tierlistRoomStore.ts'

const props = defineProps<{
  card: RoomCard
  isDragging?: boolean
}>()

const roomStore = useTierlistRoomStore()
const showNameBubble = ref(false)
const timeoutId = ref<number | null>(null)

const isDraggedByAnotherUser = computed(() => {
  return props.card.isDragged && props.card.draggedBy?.id !== roomStore.roomUser?.id
})

const toggleNameBubble = () => {
  if (props.isDragging) return

  showNameBubble.value = true
  if (timeoutId.value !== null) {
    window.clearTimeout(timeoutId.value)
  }

  timeoutId.value = window.setTimeout(() => {
    showNameBubble.value = false
  }, 2000)
}

// Masque la bulle si drag commence
watch(
  () => props.isDragging,
  (val) => {
    if (val) showNameBubble.value = false
  },
)
</script>

<template>
  <div
    class="w-19 h-19 rounded-md flex items-center justify-center bg-cover bg-center bg-no-repeat text-white font-bold relative transition-transform item-card"
    :style="{
      backgroundImage: `url(${card.image})`,
      border: isDraggedByAnotherUser ? `solid 2px ${card.draggedBy!.color}` : 'none',
    }"
    :draggable="!isDraggedByAnotherUser"
    @click="toggleNameBubble"
  >
    <div v-if="showNameBubble" class="name-bubble" @click.stop>
      {{ card.name }}
    </div>

    <div v-if="isDraggedByAnotherUser" class="avatar-indicator">
      <RoomUserAvatar :user="card.draggedBy!" size="small" />
    </div>
  </div>
</template>

<style scoped>
.item-card {
  cursor: grab;
  user-select: none;
  transition:
    transform 0.15s ease,
    box-shadow 0.15s ease;
  position: relative;
}

.item-card:active {
  cursor: grabbing;
  transform: scale(1.05);
}

@keyframes pulse {
  0% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
  100% {
    opacity: 0.6;
  }
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

.avatar-indicator {
  position: absolute;
  transform: translateX(100%);
  top: -10px;
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
