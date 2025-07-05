import { defineStore } from 'pinia'
import type { SearchResult, Template, TierList } from '@/domain/interfaces/TierList'
import api from '@/infra/http/apiClient'
import { useUtilsStore } from '@/presentation/stores/utilsStore'
import { useAuthStore } from './authStore'
import { ref } from 'vue'
import ky from 'ky'

export const useTierListStore = defineStore('tierList', () => {
  // Move store usage inside the function
  const authStore = useAuthStore()
  const utilsStore = useUtilsStore()
  const showToast = utilsStore.showToast

  const currentTierlist = ref<TierList | null>(null)

  const getTierListById = async (id: string) => {
    const response = await api.get(`tierlist/${id}`)
    const data = (await response.json()) as TierList
    currentTierlist.value = data
    return data
  }

  const getTemplateById = async (id: string) => {
    const response = await api.get(`template/${id}`)
    const data = (await response.json()) as Template
    return data
  }
  const getUserTemplates = async (userId: string) => {
    const response = await api.get(`template/user/${userId}`)
    const data = (await response.json()) as Template[]
    return data
  }

  const getUserTierLists = async (userId: string) => {
    const response = await api.get(`tierlist/user/${userId}`)
    const data = (await response.json()) as TierList[]
    return data
  }

  const saveTierList = async () => {
    if (!currentTierlist.value || !currentTierlist.value.id) return
    try {
      await api.put(`tierlist/${currentTierlist.value.id}`, {
        json: currentTierlist.value,
      })
      showToast('Tier list updated successfully', 'success', 2000)
    } catch (error) {
      showToast('Failed to update tier list', 'error', 2000)
    }
  }

  const initTemplate = async () => {
    const template: Template = {
      name: '',
      author: authStore.user?.id || '',
      is_public: false,
      cards: [],
      tags: [],
      grades: [
        {
          name: 'S',
          color: '#F55B5B',
        },
        {
          name: 'A',
          color: '#FF8652',
        },
        {
          name: 'B',
          color: '#FBAE56',
        },
        {
          name: 'C',
          color: '#FFE553',
        },
        {
          name: 'D',
          color: '#64EDD2',
        },
      ],

    }
    try {
      const response = await api.post('template', {
        json: template,
      })
      const data = (await response.json()) as { id: string }
      showToast('Template created successfully', 'success', 2000)
      return data.id
    } catch (error) {
      showToast('Failed to create template', 'error', 2000)
    }
  }

  const createTierList = async (template: Template, authorId: string) => {
    try {
      const tierList: TierList = {
        name: template.name,
        author: authorId,
        coverImage: template.coverImage,
        is_public: false,
        tags: template.tags,
        cards: template.cards,
        grades: template.grades.map(grade => ({
          name: grade.name,
          color: grade.color,
          cards: [],
        })),
      }
      const response = await api.post('tierlist', {
        json: tierList,
      })
      const data = await response.json()
      return data as { id: string }
    } catch (error) {
      console.error('Erreur dans createTierList:', error)
      showToast('Failed to create tier list', 'error', 2000)
      return null
    }
  }

  const updateTemplate = async (template: Template) => {
    try {
      if (!template.id) {
        return
      }

      await api.put(`template/${template.id}`, {
        json: template,
      })
    } catch (error) {
      console.error('Erreur dans updateTemplate:', error)
      showToast('Failed to update template', 'error', 2000)
    }
  }

  const uploadImages = async (formData: FormData) => {
    try {
      // Envoi
      const response = await ky.post(`${import.meta.env.VITE_API_URL}/image`, {
        body: formData,
        credentials: 'include',
      })
      const data = (await response.json()) as string[]
      showToast('Images uploaded successfully', 'success', 2000)
      return data
    } catch (error) {
      console.error('❌ Erreur dans uploadImages:', error)
      showToast('Failed to upload images', 'error', 2000)
    }
  }

  const getPublicTemplates = async () => {
    try {
      const response = await api.get('template/search?query=&page=1&per-page=20')
      const data = (await response.json()) as {page: number, per_page: number, data: Template[]}
      return data.data
    } catch (error) {
      console.error('Error fetching public templates:', error)
      showToast('Failed to fetch public templates', 'error', 2000)
      return []
    }
  }

  const deleteTemplate = async (templateId: string) => {
    try {
      await api.delete(`template/${templateId}`)
      showToast('Template deleted successfully', 'success', 2000)
    } catch (error) {
      console.error('Error deleting template:', error)
      showToast('Failed to delete template', 'error', 2000)
    }
  }

  const deleteTierList = async (tierListId: string) => {
    try {
      await api.delete(`tierlist/${tierListId}`)
      showToast('Tier list deleted successfully', 'success', 2000)
    } catch (error) {
      console.error('Error deleting tier list:', error)
      showToast('Failed to delete tier list', 'error', 2000)
    }
  }

  const searchTemplates = async (query: string, page: number = 1, perPage: number = 20) => {
    try {
      const response = await api.get(`template/search?query=${encodeURIComponent(query)}&page=${page}&per-page=${perPage}`)
      const data = (await response.json()) as SearchResult
      return data
    } catch (error) {
      console.error('Error searching templates:', error)
      showToast('Failed to search templates', 'error', 2000)
      return { page: 1, per_page: perPage, data: [] }
    }
  }

  return {
    getTierListById,
    saveTierList,
    getUserTemplates,
    getUserTierLists,
    currentTierlist,
    initTemplate,
    updateTemplate,
    uploadImages,
    getPublicTemplates,
    searchTemplates,
    createTierList,
    getTemplateById,
    deleteTemplate,
    deleteTierList,
  }
})
