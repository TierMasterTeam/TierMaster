<script setup lang="ts">
import { Search } from 'lucide-vue-next'

interface Props {
  modelValue: string
  placeholder?: string
  iconSize?: number
  containerClass?: string
  inputClass?: string
}

withDefaults(defineProps<Props>(), {
  placeholder: 'Rechercher...',
  iconSize: 20,
})

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="relative" :class="containerClass">
    <div class="absolute left-3 top-1/2 transform -translate-y-1/2">
      <Search :size="iconSize" class="text-gray-400" />
    </div>
    <input
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      type="text"
      :placeholder="placeholder"
      :class="[
        'w-full pl-10 pr-4 py-3 bg-dark-gray-custom border border-white-custom rounded-lg text-white-custom placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-light-green-custom focus:border-transparent transition-all duration-200',
        inputClass
      ]"
    />
  </div>
</template>
