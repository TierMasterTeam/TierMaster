import { createRouter, createWebHistory } from 'vue-router'
import TierListView from '../views/TierListView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'TierList',
      component: TierListView,
    },
    {
      path: '/create-template',
      name: 'create-template',
      component: () => import('../views/TemplateCreationView.vue'),
    },
  ],
})

export default router
