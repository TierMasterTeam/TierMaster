import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type TierList } from '@/domain/interfaces/TierList'

export const useTierListStore = defineStore('tierList', () => {
  const tierList = ref<TierList>(
    {
      name: '',
      tiers: [],
    }
  )
  const socket = ref<WebSocket | null>(null)

  const connectWebSocket = () => {
    // Fake WebSocket connection
    setTimeout(() => {
      tierList.value = {
        name: 'Programming Language',
        tiers: [
          {
            name: 'S',
            color: '#F55B5B',
            items: [
              { name: 'TypeScript', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/typescript/typescript-original.svg' },
              { name: 'C++', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/cplusplus/cplusplus-original.svg' },
            ],
          },
          {
            name: 'A',
            color: '#FF8652',
            items: [
              { name: 'Python', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/python/python-original.svg' },
              { name: 'Go', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/go/go-original.svg' },
            ],
          },
          {
            name: 'B',
            color: '#FBAE56',
            items: [
              { name: 'JavaScript', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/javascript/javascript-original.svg' },
              { name: 'C#', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/csharp/csharp-original.svg' },
              { name: 'Java', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/java/java-original.svg' },
              { name: 'Swift', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/swift/swift-original.svg' },
            ],
          },
          {
            name: 'C',
            color: '#FFE553',
            items: [
              { name: 'PHP', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/php/php-original.svg' },
            ],
          },
          {
            name: 'D',
            color: '#64EDD2',
            items: [
              { name: 'Ruby', img: 'https://cdn.jsdelivr.net/gh/devicons/devicon/icons/ruby/ruby-original.svg' },
            ],
          }
        ],
      }
    }, 1000)
  }

  const disconnectWebSocket = () => {
    if (socket.value) {
      socket.value.close()
    }
  }

  return { tierList, connectWebSocket, disconnectWebSocket }
})
