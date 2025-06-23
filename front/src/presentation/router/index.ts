import { createRouter, createWebHistory } from 'vue-router'
import NavBarLayout from '../layouts/NavBarLayout.vue'
import TierListView from '../views/TierListView.vue'
import { useAuthStore } from '@stores/authStore'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    // Public routes
    {
      path: '/',
      component: NavBarLayout,
      children: [
        { path: '', name: 'home', component: HomeView },
      ],
    },
    {
      path: '/tierlist/:id',
      component: NavBarLayout,
      children: [
        { path: '', name: 'tierList', component: TierListView },
      ]
    },
    // Protected routes
    {
      path: '/my-templates',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        { path: '', name: 'myTemplates', component: () => import('../views/TierListTemplatesView.vue') },
      ],
    },
    {
      path: '/my-templates/:id',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        { path: '', name: 'myTemplate', component: () => import('../views/MyTemplateView.vue') },
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
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()
  const requiresAuth = to.meta.requiresAuth || false

  const isAuthenticated = await authStore.checkAuth()

  if (requiresAuth && !isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else {
    next()
  }
})

export default router
