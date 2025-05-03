import { defineStore } from 'pinia'
import type { TierList } from '@/domain/interfaces/TierList'
import api from '@/infra/http/apiClient'
import { useUtilsStore } from '@/presentation/stores/utilsStore'
import { ref } from 'vue'


export const useTierListStore = defineStore('tierList', () => {

  const currentTierlist = ref<TierList | null>(null)

  const showToast = useUtilsStore().showToast

  const getTierListById = async (id: string) => {
    const response = await api.get(`tierlist/${id}`)
    const data = await response.json() as TierList
    currentTierlist.value = data
    return data
  }

  const saveTierList = async () => {
    if (!currentTierlist.value || !currentTierlist.value.id) return;
    try {
      await api.put(`tierlist/${currentTierlist.value.id}`, {
        json: currentTierlist.value
      });
      showToast('Tier list updated successfully', 'success', 2000)
    } catch (error) {
      showToast('Failed to update tier list', 'error', 2000)
    }
  }


  return { getTierListById, saveTierList, currentTierlist }
})
