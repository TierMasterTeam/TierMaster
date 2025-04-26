import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import ItemCard from '../ItemCard.vue';
import type { ComponentPublicInstance } from 'vue';

describe('ItemCard.vue', () => {
  const mockItem = {
    id: '1',
    name: 'Test Item',
    image: 'test-image.jpg'
  };

  const mockTier = {
    id: '1',
    name: 'S',
    color: '#FF0000',
    cards: []
  };

  let wrapper: VueWrapper<ComponentPublicInstance>;

  beforeEach(() => {
    wrapper = mount(ItemCard, {
      props: {
        item: mockItem,
        card: mockItem,
        index: 0,
        isDragging: false,
        isDraggedItem: false,
        isDropTarget: false
      }
    });
  });

  it('renders correctly', () => {
    expect(wrapper.exists()).toBe(true);
    expect(wrapper.attributes('style')).toContain(`background-image: url(${mockItem.image})`);
  });

  it('shows name bubble on click', async () => {
    expect(wrapper.find('.name-bubble').exists()).toBe(false);

    await wrapper.trigger('click');
    expect(wrapper.find('.name-bubble').exists()).toBe(true);
    expect(wrapper.find('.name-bubble').text()).toBe(mockItem.name);

    // Reset mocks and timers after test
    vi.useRealTimers();
  });

});
