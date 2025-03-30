import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import { createTestingPinia } from '@pinia/testing';
import TierList from '../TierList.vue';
import ItemCard from '../ItemCard.vue';
import { useTierListStore } from '../../stores/tierListStore';
import type { ComponentPublicInstance } from 'vue';
import type { Item } from '@/domain/interfaces/Item';
import type { Tier } from '@/domain/interfaces/TierList';

// Mock the ItemCard component
vi.mock('../ItemCard.vue', () => ({
  default: {
    name: 'ItemCard',
    props: ['item', 'tier', 'index', 'isDragging', 'isDraggedItem', 'isDropTarget'],
    template: '<div class="item-card-mock" data-testid="item-card"></div>'
  }
}));

describe('TierList.vue', () => {
  let wrapper: VueWrapper<ComponentPublicInstance>;
  let store: ReturnType<typeof useTierListStore>;

  const mockTierList = {
    id: '1',
    name: 'Games Tier List',
    tiers: [
      {
        id: '1',
        name: 'S',
        color: '#FF0000',
        items: [
          { id: '1', name: 'Game 1', img: 'game1.jpg' },
          { id: '2', name: 'Game 2', img: 'game2.jpg' }
        ]
      },
      {
        id: '2',
        name: 'A',
        color: '#00FF00',
        items: [
          { id: '3', name: 'Game 3', img: 'game3.jpg' }
        ]
      }
    ]
  };

  beforeEach(() => {
    wrapper = mount(TierList, {
      global: {
        plugins: [
          createTestingPinia({
            createSpy: vi.fn,
            initialState: {
              tierList: { tierList: mockTierList }
            },
          }),
        ]
      }
    });

    // Get the store instance
    store = useTierListStore();
  });

  it('renders correctly with tierList data', () => {
    // Check title renders
    expect(wrapper.find('h1').text()).toContain(mockTierList.name);

    // Check tier rows rendered
    const tierContainers = wrapper.findAll('.tier-container');
    expect(tierContainers.length).toBe(mockTierList.tiers.length);

    // Check tier names
    expect(tierContainers[0].text()).toContain(mockTierList.tiers[0].name);
    expect(tierContainers[1].text()).toContain(mockTierList.tiers[1].name);

    // Check item cards rendered
    const itemCards = wrapper.findAllComponents('[data-testid="item-card"]');
    expect(itemCards.length).toBe(
      mockTierList.tiers[0].items.length + mockTierList.tiers[1].items.length
    );
  });

  it('connects and disconnects the WebSocket', () => {
    // Check that websocket was connected on mount
    expect(store.connectWebSocket).toHaveBeenCalledTimes(1);

    // Destroy the component
    wrapper.unmount();

    // Check that websocket was disconnected
    expect(store.disconnectWebSocket).toHaveBeenCalledTimes(1);
  });

  it('handles drag and drop operations', async () => {
    // Mock dragstart event from the first item in S tier
    const dragEvent = new Event('dragstart');
    Object.defineProperty(dragEvent, 'dataTransfer', {
      value: {
        setData: vi.fn(),
        setDragImage: vi.fn()
      }
    });
    Object.defineProperty(dragEvent, 'target', {
      value: document.createElement('div')
    });

    // Find the first ItemCard component
    const firstItemCard = wrapper.findComponent(ItemCard);

    // Trigger a dragstart event with our mocked item and tier
    await firstItemCard.vm.$emit('dragstart',
      mockTierList.tiers[0].items[0],
      mockTierList.tiers[0],
      dragEvent
    );

    // Trigger dragover on a different tier
    const dragOverEvent = new Event('dragover');
    await firstItemCard.vm.$emit(
      'dragover',
      dragOverEvent,
      0, // index
      mockTierList.tiers[1] // A tier
    );

    // Trigger drop on the target tier
    await firstItemCard.vm.$emit(
      'drop',
      mockTierList.tiers[1], // drop in A tier
      0 // at index 0
    );

    // Verify the original state was reset after drag operations
    // Type assertion to access component data properties safely
    const vm = wrapper.vm as ComponentPublicInstance & {
      draggedItem: Item | null;
      draggedFromTier: Tier | null;
      isDragging: boolean;
    };

    expect(vm.draggedItem).toBe(null);
    expect(vm.draggedFromTier).toBe(null);
    expect(vm.isDragging).toBe(false);
  });
});
