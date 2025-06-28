import { beforeEach, describe, expect, it, vi } from 'vitest'
import { mount, VueWrapper } from '@vue/test-utils'
import ItemCard from '../ItemCard.vue'
import type { ComponentPublicInstance } from 'vue'
import { createTestingPinia } from '@pinia/testing'
import { useTierlistRoomStore } from '@stores/tierlistRoomStore.ts'

describe('ItemCard.vue', () => {
  const mockItem = {
    name: 'Test Item',
    image: 'test-image.jpg',
    isDragged: false,
    draggedBy: undefined,
  }

  let wrapper: VueWrapper<ComponentPublicInstance>

  beforeEach(() => {
    wrapper = mount(ItemCard, {
      global: {
        plugins: [createTestingPinia({ createSpy: vi.fn })],
      },
      props: {
        item: mockItem,
        card: mockItem,
        index: 0,
        isDragging: false,
        isDraggedItem: false,
        isDropTarget: false,
      },
    })

    useTierlistRoomStore()
  })

  it('renders correctly', () => {
    expect(wrapper.exists()).toBe(true)
    expect(wrapper.attributes('style')).toContain(`background-image: url(${mockItem.image})`)
  })

  it('shows name bubble on click', async () => {
    expect(wrapper.find('.name-bubble').exists()).toBe(false)

    await wrapper.trigger('click')
    expect(wrapper.find('.name-bubble').exists()).toBe(true)
    expect(wrapper.find('.name-bubble').text()).toBe(mockItem.name)

    // Reset mocks and timers after test
    vi.useRealTimers()
  })
})
