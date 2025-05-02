import { createRouter, createWebHistory } from 'vue-router'
import NavBarLayout from '../layouts/NavBarLayout.vue'
import TierListView from '../views/TierListView.vue'
import { useAuthStore } from '@stores/authStore'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // Public routes
    {
      path: '/',
      component: NavBarLayout,
      children: [
        { path: '', name: 'home', component: TierListView },
      ],
    },
    // Protected routes
    {
      path: '/create-template',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        { path: '', name: 'createTemplate', component: () => import('../views/TemplateCreationView.vue') },
      ],
    },
    {
      path: '/privacy-policy',
      component: NavBarLayout,
      children: [
        { path: '', name: 'privacyPolicy', component: () => import('../views/PrivacyPolicyView.vue') },
      ],
    },
    {
      path: '/terms-of-use',
      component: NavBarLayout,
      children: [
        { path: '', name: 'termsOfUse', component: () => import('../views/TermsOfUseView.vue') },
      ],
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/RegisterView.vue'),
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'NotFound',
      component: () => import('../views/NotFoundView.vue'),
    },
  ],
})

// navigation guard
router.beforeEach(async (to, from) => {
  const authStore = useAuthStore()
  
  if (to.meta.requiresAuth) {
    const isAuthenticated = await authStore.checkAuth()
    
    if (!isAuthenticated) {
      return { name: 'login', query: { redirect: to.fullPath } }
    }
  }
})

export default router
