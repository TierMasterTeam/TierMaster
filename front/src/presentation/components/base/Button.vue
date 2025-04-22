<script setup lang="ts">
import { computed } from 'vue'
import { Plus, ImageUp, Rows3 } from 'lucide-vue-next'

// Props types
type Size = 'sm' | 'md' | 'lg'
type ButtonType = 'button' | 'submit' | 'reset'
type IconName = 'plus' | 'image-up' | 'rows-3' | ''

const props = withDefaults(defineProps<{
  type?: ButtonType
  size?: Size
  icon?: IconName
  variant?: 'primary' | 'secondary' | 'danger'
}>(), {
  type: 'button',
  size: 'md',
  variant: 'primary',
  icon: '',
})

// Icon mapping
const iconMap: Record<IconName, any> = {
  'plus': Plus,
  'image-up': ImageUp,
  'rows-3': Rows3,
  '': null,
}

// Using computed for icons
const IconComponent = computed(() => iconMap[props.icon])

// Explicit definition of events
const emit = defineEmits(['click'])
</script>

<template>
  <button
    :type="type"
    :class="[
      'flex text-white-custom cursor-pointer items-center gap-2 rounded-lg px-4 py-2 border-b-2 border-r-2 active:border-none transition',
      {
        'text-sm py-1 px-3': size === 'sm',
        'text-base py-2 px-4': size === 'md',
        'text-xl py-3 px-5': size === 'lg',
        'bg-dark-green-custom border-light-green-custom': variant === 'primary',
        'bg-light-gray-custom border-dark-green-custom': variant === 'secondary',
        'bg-red-custom border-white-custom': variant === 'danger',
      }
    ]"
    @click="$emit('click')"
  >
    <component v-if="IconComponent" :is="IconComponent" class="w-5 h-5" />
    <slot />
  </button>
</template>
