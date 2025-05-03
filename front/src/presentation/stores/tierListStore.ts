import { defineStore } from 'pinia'
import type { TierList } from '@/domain/interfaces/TierList'
import api from '@/infra/http/apiClient'
import { useUtilsStore } from '@/presentation/stores/utilsStore'
import { useAuthStore } from './authStore'
import { ref } from 'vue'

export const useTierListStore = defineStore('tierList', () => {
  // Move store usage inside the function
  const authStore = useAuthStore()
  const user = authStore.user
  const utilsStore = useUtilsStore()
  const showToast = utilsStore.showToast

  const currentTierlist = ref<TierList | null>(null)

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

  const initTemplate = async () => {

    const template: TierList = {
      name: '',
      author: user?.id || '',
      cards: [],
      tags: [],
      grades: [{
        name: 'S',
        color: '#F55B5B',
        cards: [],
      },
      {
        name: 'A',
        color: '#FF8652',
        cards: [],
      },
      {
        name: 'B',
        color: '#FBAE56',
        cards: [],
      },
      {
        name: 'C',
        color: '#FFE553',
        cards: [],
      },
      {
        name: 'D',
        color: '#64EDD2',
        cards: [],
      }
      ],
    }
    try {
      console.log('Creating new template:', template)
      const response = await api.post('tierlist', {
        json: template
      });
      const data = await response.json() as string
      showToast('Template created successfully', 'success', 2000)
      return data
    } catch (error) {
      showToast('Failed to create template', 'error', 2000)
    }
  }

  const updateTemplate = async (template: TierList) => {
    try {
      const response = await api.put('tierlist', {
        json: template
      });
      await response.json()
      showToast('Template updated successfully', 'success', 2000)
    } catch (error) {
      showToast('Failed to update template', 'error', 2000)
    }
  }

  const uploadImages = async (formData: FormData) => {
    try {
      const response = await api.post('image', {
        body: formData
      });
      const data = await response.json() as string[]
      showToast('Images uploaded successfully', 'success', 2000)
      return data
    } catch (error) {
      showToast('Failed to upload images', 'error', 2000)
    }
  }

  return { getTierListById, saveTierList, currentTierlist, initTemplate, updateTemplate, uploadImages }
})
