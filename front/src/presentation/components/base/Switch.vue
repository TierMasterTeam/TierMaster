<script setup lang="ts">
import { ref, watch } from 'vue';

interface Props {
  defaultState?: boolean;
  onChange?: (state: boolean) => void;
  label?: string;
}

const props = withDefaults(defineProps<Props>(), {
  defaultState: false,
  label: 'Switch'
});

const isChecked = ref(props.defaultState);

watch(isChecked, (newState) => {
  if (props.onChange) {
    props.onChange(newState);
  }
});

</script>

<template>
    <label class="inline-flex items-center me-5 cursor-pointer">
        <input 
            type="checkbox" 
            v-model="isChecked"
            class="sr-only peer"
        >
        <div
            class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-focus:ring-4 peer-focus:ring-teal-300 dark:peer-focus:ring-teal-800 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-teal-600 dark:peer-checked:bg-teal-600">
        </div>
        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-gray-300">{{ label }}</span>
    </label>
</template>