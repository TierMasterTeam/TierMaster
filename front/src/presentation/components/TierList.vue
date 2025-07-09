<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useTierListStore } from '../stores/tierListStore'
import { type SortableEvent, VueDraggable } from 'vue-draggable-plus'
import ItemCard from '../components/ItemCard.vue'
import Button from './base/Button.vue'
import { useTierlistRoomStore } from '@stores/tierlistRoomStore.ts'
import RoomUserAvatar from '@components/RoomUserAvatar.vue'
import type { RoomCard } from '@interfaces/RoomTierlist.ts'
import { useRoute, useRouter } from 'vue-router'
import { useUtilsStore } from '../stores/utilsStore'
import { useAuthStore } from '../stores/authStore'
import { Share2 } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const showToast = useUtilsStore().showToast

const CARDS_BANK_ID = 'cards_bank'

const router = useRouter()

const roomStore = useTierlistRoomStore()

const isDragging = ref(false)

const route = useRoute()
const tierlistId = route.params.id as string || ''

const { t } = useI18n();

onMounted(() => {
  if (!tierlistId) {
    console.error('Tierlist ID is required to join the room')
    showToast(t('tierlist.tierlistIdRequired'), 'error', 2000)
    router.push({ name: 'home' })
    return
  }
  roomStore.join(tierlistId)
})

const getCardFromEvent = (event: SortableEvent, index: number) => {
  if (
    (event.to && event.to.id === CARDS_BANK_ID) ||
    (!event.to && event.from.id === CARDS_BANK_ID)
  ) {
    return roomStore.tierlist!.cards[index]
  }

  const gradeId = event.to?.id || event.from.id
  const gradeIndex = parseGradeIndexFromGradeId(gradeId)
  return roomStore.tierlist!.grades[gradeIndex].cards[index]
}

const updateCardDragState = (card: RoomCard, isDragged: boolean) => {
  card.isDragged = isDragged
  card.draggedBy = isDragged ? roomStore.roomUser! : undefined
}

const onChoose = (event: SortableEvent) => {
  const card = getCardFromEvent(event, event.oldIndex!)
  updateCardDragState(card, true)
  roomStore.updateTierlist(roomStore.tierlist!)
}

const onUnchoose = (event: SortableEvent) => {
  if (event.from !== event.to) return // skip if dropped elsewhere (handled in onDragEnd)

  const card = getCardFromEvent(event, event.oldIndex!)
  updateCardDragState(card, false)
  roomStore.updateTierlist(roomStore.tierlist!)
}

const onDragStart = () => {
  isDragging.value = true
}

function parseGradeIndexFromGradeId(id: string) {
  const gradeIdElements = id.split('_')
  return parseInt(gradeIdElements[gradeIdElements.length - 1])
}

const onDragEnd = (event: SortableEvent) => {
  isDragging.value = false

  if (!event.to?.id) return

  const card = getCardFromEvent(event, event.newIndex!)
  updateCardDragState(card, false)
  roomStore.updateTierlist(roomStore.tierlist!)
}

const shareUrl = async () => {
  try {
    const currentUrl = window.location.href
    await navigator.clipboard.writeText(currentUrl)
    showToast(t('tierlist.urlCopied'), 'success', 2000)
  } catch (error) {
    console.error('Erreur lors de la copie de l\'URL:', error)
    showToast(t('tierlist.urlCopyError'), 'error', 2000)
  }
}
</script>

<template>
  <div class="container mx-auto p-4 pb-8" v-if="roomStore.tierlist">
    <div class="flex justify-between items-center gap-2 p-2">
      <Button
        variant="secondary"
        size="md"
        @click="shareUrl"
        class="flex items-center gap-2"
      >
        <Share2 class="w-4 h-4" />
         {{ $t('tierlist.share') }}
      </Button>

      <div class="flex gap-2">
        <div v-for="user in roomStore.users" :key="user.id">
          <RoomUserAvatar :user="user" size="large" />
        </div>
      </div>
    </div>

    <h1 v-if="roomStore.tierlist.name" class="text-4xl font-bold text-[#31E7C3] pb-4 font-jersey">
      {{ roomStore.tierlist.name }} :
    </h1>

    <div class="grid gap-4">
      <div
        v-for="(grade, index) in roomStore.tierlist.grades"
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
          :id="`grade_${grade.name}_${index}`"
          group="grades"
          class="flex-1 flex flex-wrap gap-2 rounded-md items-center"
          :on-start="onDragStart"
          :on-end="onDragEnd"
          :onChoose="onChoose"
          :onUnchoose="onUnchoose"
        >
          <div v-for="card in grade.cards" :key="card.name" class="w-19 h-19">
            <ItemCard :card="card" :is-dragging="isDragging" />
          </div>
        </VueDraggable>
      </div>
      <VueDraggable
        v-model="roomStore.tierlist.cards"
        :id="CARDS_BANK_ID"
        group="grades"
        class="flex-1 flex flex-wrap gap-2 rounded-md items-center"
        :on-start="onDragStart"
        :on-end="onDragEnd"
        :onChoose="onChoose"
        :onUnchoose="onUnchoose"
      >
        <div v-for="card in roomStore.tierlist.cards" :key="card.name" class="w-19 h-19">
          <ItemCard :card="card" :is-dragging="isDragging" />
        </div>
      </VueDraggable>
    </div>

  </div>
  <div class="flex flex-1 items-center justify-center w-full p-4" v-else-if="roomStore.connecting">
    <p class="text-center text-2xl"></p>
  </div>
</template>

<style scoped>
.tier-container {
  transition:
    box-shadow 0.3s ease,
    border 0.3s ease;
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
