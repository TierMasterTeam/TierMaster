<script setup lang="ts">
// Props types
type Size = 'sm' | 'md' | 'lg'
type ButtonType = 'button' | 'submit' | 'reset'

const props = withDefaults(defineProps<{
  type?: ButtonType
  size?: Size
  variant?: 'primary' | 'secondary' | 'danger'
}>(), {
  type: 'button',
  size: 'md',
  variant: 'primary',
})

// Explicit definition of events
const emit = defineEmits(['click'])
</script>

<template>
  <button
    :type="type"
    :class="[
      'flex text-white-custom cursor-pointer items-center gap-2 rounded-lg px-4 py-2 border-b-2 border-r-2 active:border-transparent active:translate-x-[2px] active:translate-y-[2px] transition font-bold',
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
    <slot name="icon"></slot>
    <slot />
  </button>
</template>
