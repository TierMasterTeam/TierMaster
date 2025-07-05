<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center" @click.self="cancel">
    <div class="absolute inset-0 bg-dark-gray-custom/10 backdrop-blur-sm" @click="cancel"></div>
    <div class="relative bg-dark-gray-custom border border-white-custom rounded-lg shadow-lg p-8 min-w-[320px] z-10 flex flex-col items-center">
      <button class="absolute top-2 right-2 text-gray-400 hover:text-gray-600" @click="cancel" aria-label="Fermer">
        <X :size="20" />
      </button>
      <slot name="title">
        <h2 class="text-xl font-bold mb-4">Confirmer la suppression</h2>
      </slot>
      <slot name="content">
        <p class="mb-6">Êtes-vous sûr de vouloir supprimer cet élément ?</p>
      </slot>
      <div class="flex gap-4 w-full justify-center">
        <Button size="md" variant="secondary" @click="cancel">Annuler</Button>
        <Button size="md" :variant="confirmVariant" @click="confirm">Confirmer</Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue'
import Button from './Button.vue'
import { X } from 'lucide-vue-next'

const props = defineProps({
  show: Boolean,
  confirmVariant: {
    type: String as () => 'danger' | 'primary' | 'secondary',
    default: 'danger'
  }
})
const emit = defineEmits(['confirm', 'cancel'])

function confirm() {
  emit('confirm')
}
function cancel() {
  emit('cancel')
}
</script>
