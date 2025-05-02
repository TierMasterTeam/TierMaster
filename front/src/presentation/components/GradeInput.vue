<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import { Pipette, Trash2 } from 'lucide-vue-next'

const props = defineProps<{
  modelValue: string
  color: string
}>()
const emit = defineEmits(['update:modelValue', 'update:color', 'delete'])

const textareaRef = ref<HTMLTextAreaElement | null>(null)

const autoResize = () => {
  if (textareaRef.value) {
    textareaRef.value.style.height = 'auto'
    textareaRef.value.style.height = textareaRef.value.scrollHeight + 'px'
  }
}

watch(() => props.modelValue, () => {
  nextTick(autoResize)
})

onMounted(() => {
  autoResize()
})

function getContrastTextColor(hex: string): string {
  hex = hex.replace('#', '')
  const r = parseInt(hex.substring(0, 2), 16)
  const g = parseInt(hex.substring(2, 4), 16)
  const b = parseInt(hex.substring(4, 6), 16)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255
  return luminance > 0.5 ? '#1A1C1E' : '#F7F7F8'
}
</script>

<template>
  <div class="w-[76px] h-auto min-h-[76px] rounded-xl flex flex-col items-center justify-center relative gap-1 group"
    :style="{ backgroundColor: color }">
    <button
      class="absolute -top-3 -left-3 z-10 w-8 h-8 flex items-center justify-center rounded-full bg-gray-custom/30 backdrop-blur-3xl transition-opacity duration-200 opacity-0 group-hover:opacity-100"
      @click="$emit('delete')"
      type="button"
      tabindex="-1"
    >
      <Trash2 class="text-white-custom" :size="16" />
    </button>
    <div class="absolute -top-3 -right-3 z-10 w-8 h-8 flex items-center justify-center rounded-full bg-gray-custom/30 backdrop-blur-3xl transition-opacity duration-200 opacity-0 group-hover:opacity-100">
      <input type="color" :value="color" @input="e => emit('update:color', (e.target as HTMLInputElement).value)"
        class="w-8 h-8 p-0 border-none cursor-pointer opacity-0 absolute inset-0" />
      <Pipette class="text-white-custom" :size="16" />
    </div>
    <textarea ref="textareaRef" :value="modelValue"
      @input="e => emit('update:modelValue', (e.target as HTMLTextAreaElement).value)"
      class="text-center bg-transparent resize-none w-full p-0 border-none focus:ring-0 focus:outline-none"
      :style="{ color: getContrastTextColor(color) }"
      rows="1" @input.native="autoResize" placeholder="Type..." />
  </div>
</template>
