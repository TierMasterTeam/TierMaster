import { createRouter, createWebHistory } from 'vue-router'
import NavBarLayout from '../layouts/NavBarLayout.vue'
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
        {
          path: '',
          name: 'home',
          component: HomeView,
          meta: {
            title: 'TierMaster - Accueil',
            description: 'TierMaster, votre plateforme de tier list dynamique et personnalisée avec vos amis.'
          }
        }
      ],
    },
    // Protected routes
    {
      path: '/my-templates',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'myTemplates',
          component: () => import('../views/TierListTemplatesView.vue'),
          meta: {
            title: 'Mes Modèles de Tier List | TierMaster',
            description: 'Consultez et gérez vos modèles de tier lists personnels.'
          }
        },
      ],
    },
    {
      path: '/my-templates/:id',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'myTemplate',
          component: () => import('../views/MyTemplateView.vue'),
          meta: {
            title: 'Modèle | TierMaster',
            description: 'Visualisez et éditez un modèle de tier list spécifique.'
          }
        },
      ],
    },
    {
      path: '/my-tierlists',
      component: NavBarLayout,
      meta: { requiresAuth: true },
      children: [
        {
          path: '',
          name: 'myTierlists',
          component: () => import('../views/TierListView.vue'),
          meta: {
            title: 'Mes Tier Lists | TierMaster',
            description: 'Accédez et jouer a nouveau à vos tier lists déjà créées.'
          }
        },
      ],
    },
    {
      path: '/tierlist/:id',
      component: NavBarLayout,
      meta: { requiresAuth: false },
      children: [
        {
          path: '',
          name: 'tierlist',
          component: () => import('../views/TierListView.vue'),
          meta: {
            title: 'Tier List | TierMaster',
            description: 'Collaborez, modifier la tierlist'
          }
        },
      ],
    },
    {
      path: '/privacy-policy',
      component: NavBarLayout,
      children: [
        {
          path: '',
          name: 'privacyPolicy',
          component: () => import('../views/PrivacyPolicyView.vue'),
          meta: {
            title: 'Politique de Confidentialité | TierMaster',
            description: 'Consultez notre politique de confidentialité.'
          }
        },
      ],
    },
    {
      path: '/terms-of-use',
      component: NavBarLayout,
      children: [
        {
          path: '',
          name: 'termsOfUse',
          component: () => import('../views/TermsOfUseView.vue'),
          meta: {
            title: "Conditions d'Utilisation | TierMaster",
            description: "Lisez attentivement nos conditions d'utilisation."
          }
        },
      ],
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue'),
      meta: {
        title: 'Connexion | TierMaster',
        description: 'Connectez-vous pour accéder à vos fonctionnalités.'
      }
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/RegisterView.vue'),
      meta: {
        title: 'Inscription | TierMaster',
        description: 'Créez un compte pour commencer à utiliser notre application.'
      }
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'NotFound',
      component: () => import('../views/NotFoundView.vue'),
      meta: {
        title: 'Page Non Trouvée | TierMaster',
        description: "La page que vous recherchez n'existe pas."
      }
    },
  ],
})

// navigation guard: auth
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()
  const requiresAuth = to.meta.requiresAuth || false
  const isAuthenticated = await authStore.checkAuth()

  // SEO dynamique
  const meta = to.meta
  if (meta.title) document.title = meta.title as string
  const desc = document.querySelector('meta[name="description"]')
  if (desc && meta.description) desc.setAttribute('content', meta.description as string)

  if (requiresAuth && !isAuthenticated) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else {
    next()
  }
})

export default router
