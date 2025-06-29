<script setup lang="ts">
import { ref, computed } from 'vue';
import TemplateCard from './TemplateCard.vue';
import type { Template } from '@/domain/interfaces/TierList';

const props = defineProps<{ templates: Template[] }>();

const currentIndex = ref(0);

import { onMounted, onUnmounted } from 'vue';
import { CircleChevronLeft, CircleChevronRight } from 'lucide-vue-next';
const visibleCount = ref(4);

function updateVisibleCount() {
  const w = window.innerWidth;
  if (w < 640) visibleCount.value = 1;
  else if (w < 900) visibleCount.value = 2;
  else if (w < 1200) visibleCount.value = 3;
  else visibleCount.value = 4;
}

onMounted(() => {
  updateVisibleCount();
  window.addEventListener('resize', updateVisibleCount);
});
onUnmounted(() => {
  window.removeEventListener('resize', updateVisibleCount);
});

const gap = 16;
const cardWidth = 288;

const carouselRef = ref<HTMLDivElement | null>(null);


const canGoLeft = computed(() => currentIndex.value > 0);
const canGoRight = computed(() => currentIndex.value + visibleCount.value < props.templates.length);

const visibleTemplates = computed(() => {
  return props.templates.slice(currentIndex.value, currentIndex.value + visibleCount.value);
});

// Touch scroll logic
let startX = 0;
let deltaX = 0;
let isDragging = false;

function onTouchStart(e: TouchEvent) {
  isDragging = true;
  startX = e.touches[0].clientX;
}

function onTouchMove(e: TouchEvent) {
  if (!isDragging) return;
  deltaX = e.touches[0].clientX - startX;
}

function onTouchEnd() {
  if (!isDragging) return;
  if (deltaX > 50 && canGoLeft.value) {
    goLeft();
  } else if (deltaX < -50 && canGoRight.value) {
    goRight();
  }
  isDragging = false;
  deltaX = 0;
}

function goLeft() {
  if (canGoLeft.value) {
    currentIndex.value -= visibleCount.value;
    if (currentIndex.value < 0) currentIndex.value = 0;
  }
}
function goRight() {
  if (canGoRight.value) {
    currentIndex.value += visibleCount.value;
    if (currentIndex.value > props.templates.length - visibleCount.value) {
      currentIndex.value = Math.max(props.templates.length - visibleCount.value, 0);
    }
  }
}
</script>

<template>
  <div class="flex flex-col items-center w-full">
    <div class="flex items-center w-fit justify-center gap-2 relative">
      <button @click="goLeft" :disabled="!canGoLeft" class="p-2 text-2xl absolute right-full " aria-label="Previous">
        <CircleChevronLeft class="w-6 h-6" :class="{'opacity-30': !canGoLeft}" />
      </button>

      <div class="relative w-[288px] grid2:w-[592px] grid3:w-[896px] grid4:w-[1200px] overflow-hidden">
        <div
          class="flex gap-4 transition-transform duration-300 ease-in-out"
          :style="{ transform: `translateX(-${currentIndex * (cardWidth + gap)}px)` }"
          ref="carouselRef"
        >
          <TemplateCard
            v-for="template in props.templates"
            :key="template.id"
            :template="template"
            class="shrink-0"
          />
        </div>
      </div>

      <button @click="goRight" :disabled="!canGoRight" class="p-2 text-2xl absolute left-full" aria-label="Next">
        <CircleChevronRight class="w-6 h-6" :class="{'opacity-30': !canGoRight}" />
      </button>
    </div>

  </div>
</template>

