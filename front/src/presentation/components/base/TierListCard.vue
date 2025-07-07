<template>
  <div class="bg-dark-gray-custom border border-white-custom rounded-lg overflow-hidden hover:border-light-green-custom transition-all duration-300 cursor-pointer group" @click="onClick">
    <div class="relative h-48 bg-gray-700">
      <img
        v-if="coverImage"
        :src="coverImage"
        :alt="title"
        class="w-full h-full object-cover"
      />
      <div v-else class="w-full h-full flex items-center justify-center">
        <FileText :size="48" class="text-gray-400" />
      </div>
    </div>

    <div class="p-4">
      <h3 class="text-lg font-semibold text-white-custom mb-2 line-clamp-2 group-hover:text-light-green-custom transition-colors">
        {{ title || $t('tierlist.untitled') }}
      </h3>

      <div v-if="tags && tags.length > 0" class="flex flex-wrap gap-1 mb-3">
        <span
          v-for="tag in tags.slice(0, 3)"
          :key="tag"
          class="px-2 py-1 text-xs bg-gray-600 text-gray-200 rounded-full"
        >
          {{ tag }}
        </span>
        <span v-if="tags.length > 3" class="px-2 py-1 text-xs bg-gray-600 text-gray-200 rounded-full">
          +{{ tags.length - 3 }}
        </span>
      </div>

      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2 text-sm text-gray-400">
          <Users :size="16" />
          <span>{{ $t('tierlist.items', { count: itemCount }) }}</span>
        </div>

        <div class="flex gap-2">
          <button
            v-if="showDelete"
            @click.stop="onDelete"
            class="p-2 text-gray-400 hover:text-red-400 transition-colors"
            :title="$t('tierlist.delete')"
          >
            <Trash2 :size="16" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { FileText, Users, Trash2 } from 'lucide-vue-next'

interface Props {
  id: string
  title?: string
  coverImage?: string
  isPublic?: boolean
  tags?: string[]
  itemCount?: number
  showEdit?: boolean
  showDelete?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '',
  isPublic: false,
  tags: () => [],
  itemCount: 0,
  showEdit: true,
  showDelete: true
})

const emit = defineEmits<{
  click: [id: string]
  delete: [id: string]
}>()

const onClick = () => {
  emit('click', props.id)
}

const onDelete = () => {
  emit('delete', props.id)
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
