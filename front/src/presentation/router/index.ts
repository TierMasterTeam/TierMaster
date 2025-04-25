import { createRouter, createWebHistory } from 'vue-router'
import NavBarLayout from '@/layouts/NavBarLayout.vue'

import TierListView from '../views/TierListView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: NavBarLayout,
      children: [
        { path: '', name: 'home', component: TierListView },
      ],
    },
    {
      path: '/create-template',
      component: NavBarLayout,
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

export default router
