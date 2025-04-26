import { defineStore } from 'pinia'
import type { TierList } from '@/domain/interfaces/TierList'
import api from '@/infra/http/apiClient'
import { ref } from 'vue'

export const useTierListStore = defineStore('tierList', () => {

  const currentTierlist = ref<TierList | null>(null)

  const getTierListById = async (id: string) => {
    const response = await api.get(`tierlist/${id}`)
    const data = await response.json() as TierList
    currentTierlist.value = data
    return data
  }

  return { getTierListById , currentTierlist }
})
