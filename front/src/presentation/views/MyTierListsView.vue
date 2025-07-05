<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/authStore'
import { useTierListStore } from '../stores/tierListStore'
import type { TierList } from '@/domain/interfaces/TierList'
import TierListCard from '../components/base/TierListCard.vue'
import ConfirmPopup from '../components/base/ConfirmPopup.vue'
import SearchBar from '../components/base/SearchBar.vue'
import Button from '../components/base/Button.vue'
import { Plus } from 'lucide-vue-next'

const router = useRouter()
const authStore = useAuthStore()
const tierListStore = useTierListStore()

const user = authStore.user
const tierLists = ref<TierList[]>([])
const searchQuery = ref('')
const filteredTierLists = ref<TierList[]>([])
const isLoading = ref(true)

// Popup de suppression
const showDeletePopup = ref(false)
const tierListToDelete = ref<TierList | null>(null)

onMounted(async () => {
  if (!user) {
    console.error('User is not authenticated')
    router.push('/login')
    return
  }

  try {
    isLoading.value = true
    tierLists.value = await tierListStore.getUserTierLists(user.id)
    filteredTierLists.value = tierLists.value
  } catch (error) {
    console.error('Error loading tier lists:', error)
  } finally {
    isLoading.value = false
  }
})

// Filtrage par recherche
const filterTierLists = () => {
  if (!searchQuery.value.trim()) {
    filteredTierLists.value = tierLists.value
    return
  }

  const query = searchQuery.value.toLowerCase()
  filteredTierLists.value = tierLists.value.filter(tierList =>
    tierList.name?.toLowerCase().includes(query) ||
    tierList.tags?.some(tag => tag.toLowerCase().includes(query))
  )
}

// Actions
const createNewTierList = () => {
  // Rediriger vers la page de création de tier list ou de choix de template
  router.push('/my-templates')
}

const openTierList = (id: string) => {
  router.push(`/tierlist/${id}`)
}

const openDeletePopup = (tierList: TierList) => {
  tierListToDelete.value = tierList
  showDeletePopup.value = true
}

const closeDeletePopup = () => {
  showDeletePopup.value = false
  tierListToDelete.value = null
}

const confirmDelete = async () => {
  if (tierListToDelete.value) {
    try {
      await tierListStore.deleteTierList(tierListToDelete.value.id!)
      // Supprimer localement après succès de l'API
      tierLists.value = tierLists.value.filter(t => t.id !== tierListToDelete.value?.id)
      filterTierLists()
    } catch (error) {
      console.error('Error deleting tier list:', error)
    }
  }
  closeDeletePopup()
}

const getItemCount = (tierList: TierList): number => {
  return tierList.cards?.length || 0
}
</script>

<template>
  <div class="flex flex-col p-16 w-full">
    <div class="flex items-center justify-between mb-8">
      <div>
        <h1 class="text-[40px] font-jersey text-white-custom">
          {{ $t('tierlist.myTierLists') }}
        </h1>
        <p class="text-gray-300 mt-2">
          {{ $t('tierlist.subtitle') }}
        </p>
      </div>

      <Button size="md" variant="primary" @click="createNewTierList" class="flex items-center gap-2">
        <Plus :size="20" />
        {{ $t('tierlist.createNew') }}
      </Button>
    </div>

    <!-- Barre de recherche -->
    <SearchBar
      v-model="searchQuery"
      :placeholder="$t('tierlist.searchPlaceholder')"
      @update:model-value="filterTierLists"
    />

    <!-- Chargement -->
    <div v-if="isLoading" class="flex justify-center py-12">
      <div class="text-gray-400">{{ $t('tierlist.loading') }}</div>
    </div>

    <!-- Aucun résultat -->
    <div v-else-if="filteredTierLists.length === 0" class="text-center py-12">
      <div class="text-gray-400 mb-4">
        {{ searchQuery ? $t('tierlist.noResults') : $t('tierlist.noTierLists') }}
      </div>
      <Button v-if="!searchQuery" variant="primary" @click="createNewTierList" class="flex items-center gap-2 mx-auto">
        <Plus :size="20" />
        {{ $t('tierlist.createFirst') }}
      </Button>
    </div>

    <!-- Grille des tier lists -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mt-8">
      <TierListCard
        v-for="tierList in filteredTierLists"
        :key="tierList.id"
        :id="tierList.id!"
        :title="tierList.name"
        :cover-image="tierList.coverImage"
        :tags="tierList.tags"
        :item-count="getItemCount(tierList)"
        :show-delete="true"
        @click="openTierList"
        @delete="() => openDeletePopup(tierList)"
      />
    </div>

    <!-- Popup de confirmation de suppression -->
    <ConfirmPopup
      :show="showDeletePopup"
      confirm-variant="danger"
      @confirm="confirmDelete"
      @cancel="closeDeletePopup"
    >
      <template #title>
        <h2 class="text-xl font-bold mb-4">{{ $t('tierlist.confirmDelete') }}</h2>
      </template>
      <template #content>
        <p class="mb-6">{{ $t('tierlist.confirmDeleteMessage', { name: tierListToDelete?.name || 'cette tier list' }) }}</p>
      </template>
    </ConfirmPopup>
  </div>
</template>

<style scoped>
/* Styles additionnels si nécessaire */
</style>
