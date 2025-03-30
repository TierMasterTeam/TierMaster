import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import ItemCard from '../ItemCard.vue';
import type { ComponentPublicInstance } from 'vue';

describe('ItemCard.vue', () => {
  const mockItem = {
    id: '1',
    name: 'Test Item',
    img: 'test-image.jpg'
  };

  const mockTier = {
    id: '1',
    name: 'S',
    color: '#FF0000',
    items: []
  };

  let wrapper: VueWrapper<ComponentPublicInstance>;

  beforeEach(() => {
    wrapper = mount(ItemCard, {
      props: {
        item: mockItem,
        tier: mockTier,
        index: 0,
        isDragging: false,
        isDraggedItem: false,
        isDropTarget: false
      }
    });
  });

  it('renders correctly', () => {
    expect(wrapper.exists()).toBe(true);
    expect(wrapper.attributes('style')).toContain(`background-image: url(${mockItem.img})`);
  });

  it('applies correct classes based on props', async () => {
    // Test default state
    expect(wrapper.classes()).toContain('item-hover');

    // Test dragging state
    await wrapper.setProps({ isDraggedItem: true });
    expect(wrapper.classes()).toContain('dragging');
    expect(wrapper.attributes('style')).toContain('scale(0.9)');

    // Test drop target state
    await wrapper.setProps({ isDraggedItem: false, isDropTarget: true });
    expect(wrapper.classes()).toContain('drag-over-before');
  });

  it('shows name bubble on click', async () => {
    expect(wrapper.find('.name-bubble').exists()).toBe(false);

    await wrapper.trigger('click');
    expect(wrapper.find('.name-bubble').exists()).toBe(true);
    expect(wrapper.find('.name-bubble').text()).toBe(mockItem.name);

    // Reset mocks and timers after test
    vi.useRealTimers();
  });

  it('emits events on drag interactions', async () => {
    // Test dragstart - instead of creating a native event, use a simple object
    await wrapper.trigger('dragstart', {
      dataTransfer: { setData: vi.fn() }
    });
    const dragstartEvents = wrapper.emitted('dragstart');
    expect(dragstartEvents).toBeTruthy();
    expect(dragstartEvents?.[0][0]).toEqual(mockItem);
    expect(dragstartEvents?.[0][1]).toEqual(mockTier);
    // The event will be different but it should exist
    expect(dragstartEvents?.[0][2]).toBeTruthy();

    // Test dragover
    await wrapper.trigger('dragover', {
      preventDefault: vi.fn()
    });
    const dragoverEvents = wrapper.emitted('dragover');
    expect(dragoverEvents).toBeTruthy();
    // Check that event exists
    expect(dragoverEvents?.[0][0]).toBeTruthy();
    expect(dragoverEvents?.[0][1]).toEqual(0); // index
    expect(dragoverEvents?.[0][2]).toEqual(mockTier);

    // Test dragleave
    await wrapper.trigger('dragleave');
    expect(wrapper.emitted('dragleave')).toBeTruthy();

    // Test drop
    await wrapper.trigger('drop', {
      preventDefault: vi.fn(),
      stopPropagation: vi.fn()
    });
    const dropEvents = wrapper.emitted('drop');
    expect(dropEvents).toBeTruthy();
    expect(dropEvents?.[0]).toEqual([mockTier, 0]);

    // Test dragend
    await wrapper.trigger('dragend');
    expect(wrapper.emitted('dragend')).toBeTruthy();
  });
});
